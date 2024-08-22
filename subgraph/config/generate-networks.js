const fs = require('fs');
const yaml = require('js-yaml');

function main() {
    let ds = ['erc20', 'erc721', 'erc404']
    const configs = yaml.load(fs.readFileSync(__dirname + `/../config.yaml`, 'utf8'));

    // Process each individual YAML template
    ds.forEach((dataSource) => {
        if (configs[dataSource]) {
            // Initialize an empty object for networks.json structure
            let networksData = {};

            // Define the network key
            const networkKey = 'https://devnet-rpc.zionx.network';
            networksData[networkKey] = {};

            // Populate the networks.json structure based on config.yaml
            configs[dataSource].forEach(token => {
                networksData[networkKey][token.name] = {
                    address: token.address,
                    startBlock: token.startBlock
                };
            });

            // Specify the path for the new networks.json file
            const newNetworksPath = __dirname + `/../${dataSource}/networks.json`;

            // Save the generated networks.json
            fs.writeFileSync(newNetworksPath, JSON.stringify(networksData, null, 2));

            console.log(`${dataSource} networks.json file created successfully.`);
        }
    });
}

main()