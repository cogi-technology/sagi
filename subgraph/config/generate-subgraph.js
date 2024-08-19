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

    const arguments = process.argv.slice(2);

    if (!arguments.every(arg => ds.includes(arg)) || arguments.length === 0) {
        console.log("Invalid datasource, please provide a valid datasource name. Valid datasources are: " + ds.join(', '));
        return;
    }

    // Process each individual YAML template
    arguments.forEach((dataSource) => {
        const configs = yaml.load(fs.readFileSync(__dirname + `/dist/${dataSource}-config.yaml`, 'utf8'));
        const subgraphYaml = processTemplate(__dirname + `/templates/${dataSource}-subgraph.template.yaml`, { configs });
        fs.writeFileSync(__dirname + `/../${dataSource}/subgraph.yaml`, subgraphYaml);
    });
}

main()

// // Process each individual YAML template
// ['erc20', 'erc721', 'erc404'].forEach((dataSource) => {
//     if (config[dataSource]) {
//         const dataSourceYaml = processTemplate(__dirname + `/templates/${dataSource}.datasource.yaml`, config[dataSource]);
//         fs.writeFileSync(__dirname + `/generated/${dataSource}.yaml`, dataSourceYaml);
//     }
// });

// let datasources = [];
// ['erc20', 'erc721', 'erc404'].forEach((dataSource) => {
//     if (config[dataSource]) {
//         const dataSourceYaml = yaml.load(fs.readFileSync(__dirname + `/generated/${dataSource}.yaml`, 'utf8'));
//         datasources.push(dataSourceYaml);
//     }
// });

// let subgraphYaml = processTemplate(__dirname + '/templates/subgraph.template.yaml', { datasources });

// // // Write the final subgraph.yaml file
// fs.writeFileSync(__dirname + '/../subgraphs/subgraph.yaml', subgraphYaml);

// console.log('subgraph.yaml generated successfully.');