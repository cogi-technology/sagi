use {
    crate::{
        database::Database,
        models::{
            to_error_status,
            Bill,
            BillError,
            NewBill,
            Status,
        },
        schema::bills,
    },
    chrono::Local,
    diesel::prelude::*,
    diesel_async::RunQueryDsl,
    std::sync::Arc,
};

#[derive(Debug, Clone)]
pub struct Bills {
    db: Arc<Database>,
}

impl Bills {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get(&self, bill_id: String) -> Result<Bill, BillError> {
        let ret = bills::table
            .filter(bills::id.eq(bill_id))
            .select(Bill::as_select())
            .first(&mut self.db.get_connection().await)
            .await?;

        Ok(ret)
    }

    pub async fn get_last_by_customer(&self, customer: String) -> Result<Bill, BillError> {
        let ret = bills::table
            .filter(bills::customer.eq(customer))
            .order(bills::paid_at.desc())
            .select(Bill::as_select())
            .first(&mut self.db.get_connection().await)
            .await?;

        Ok(ret)
    }

    pub async fn add(
        &self,
        bill_id: String,
        bill_customer: String,
        bill_amount: String,
        paid_txhash: String,
    ) -> Result<Bill, BillError> {
        let bill = self.get(bill_id.clone()).await.unwrap_or(Bill::default());
        if !(bill.status == Status::NotFound) {
            return Err(BillError {
                msg: "Bills.add failed".into(),
                status: to_error_status(bill.status) as i32,
            });
        }

        let new_bill = NewBill {
            id: &bill_id,
            customer: &bill_customer,
            paid_amount: &bill_amount,
            paid_txhash: &paid_txhash,
            paid_at: &Local::now().naive_utc(),
            status: &Status::Paid.into(),
        };

        let mut conn = self.db.get_connection().await;
        let ret = diesel::insert_into(bills::table)
            .values(new_bill)
            .returning(Bill::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn update(
        &self,
        bill_id: String,
        rewarded_txhash: Option<String>,
        rewarded_amount: Option<String>,
        status: Status,
    ) -> Result<Bill, BillError> {
        let mut b = self.get(bill_id.clone()).await.unwrap_or(Bill::default());

        let is_allow_rewarding_wait_for_tx_confirmed =
            b.status == Status::Paid && status == Status::RewardingWaitForTxConfirmed;

        let is_allow_rewarded = (b.status == Status::RewardingWaitForTxConfirmed
            || b.status == Status::Paid)
            && status == Status::Rewarded;

        if !is_allow_rewarding_wait_for_tx_confirmed && !is_allow_rewarded {
            return Err(BillError {
                msg: "update failed".into(),
                status: to_error_status(b.status) as i32,
            });
        }
        if rewarded_amount.is_some() {
            b.rewarded_amount = rewarded_amount;
        }
        if rewarded_txhash.is_some() {
            b.rewarded_txhash = rewarded_txhash;
            b.rewarded_at = Some(Local::now().naive_utc())
        }
        b.status = status;

        let mut conn = &mut self.db.get_connection().await;
        let ret = diesel::update(bills::table)
            .filter(bills::id.eq(bill_id))
            .set(b)
            .returning(Bill::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn update_rewarding(&self, bill_id: String) -> Result<Bill, BillError> {
        self.update(bill_id, None, None, Status::RewardingWaitForTxConfirmed)
            .await
    }

    pub async fn update_rewarded(
        &self,
        bill_id: String,
        rewarded_txhash: Option<String>,
        rewarded_amount: Option<String>,
    ) -> Result<Bill, BillError> {
        self.update(bill_id, rewarded_txhash, rewarded_amount, Status::Rewarded)
            .await
    }

    pub async fn delete(&self, bill_id: String) -> Result<usize, BillError> {
        let mut conn = self.db.get_connection().await;
        let ret = diesel::delete(bills::table)
            .filter(bills::id.eq(bill_id))
            .execute(&mut conn)
            .await?;

        Ok(ret)
    }
}
