const fs = require('fs');
const yaml = require('js-yaml');
const Handlebars = require('handlebars');
const path = require('path');

// Helper function to process a YAML template
function processTemplate(templateFile, context) {
    const templateSource = fs.readFileSync(templateFile, 'utf8');
    const template = Handlebars.compile(templateSource);
    return template(context);
}

function main() {
    let ds = ['erc20', 'erc721', 'erc404']
    const configs = yaml.load(fs.readFileSync(__dirname + `/config.yaml`, 'utf8'))['datasources'];

    // Process each individual YAML template
    ds.forEach((dataSource) => {
        if (configs[dataSource]) {
            const subgraphYaml = processTemplate(__dirname + `/templates/${dataSource}-subgraph.template.yaml`, { configs: configs[dataSource] });
            fs.writeFileSync(__dirname + `/../${dataSource}/subgraph.yaml`, subgraphYaml);

            console.log(`${dataSource} subgraph.yaml file created successfully.`);
        }
    });
}

main()