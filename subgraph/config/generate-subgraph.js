const fs = require('fs');
const yaml = require('js-yaml');
const Handlebars = require('handlebars');
const path = require('path');

// Load config.yaml
const config = yaml.load(fs.readFileSync(__dirname + '/config.yaml', 'utf8'));

// Helper function to process a YAML template
function processTemplate(templateFile, context) {
    const templateSource = fs.readFileSync(templateFile, 'utf8');
    const template = Handlebars.compile(templateSource);
    return template(context);
}

// Define the directory you want to create
const dirPath = path.join(__dirname, 'generated');

// Create the directory
fs.mkdir(dirPath, { recursive: true }, (err) => {
    if (err) {
        return console.error(`Error creating directory: ${err.message}`);
    }
});

// Process each individual YAML template
['erc20', 'erc721', 'erc404'].forEach((dataSource) => {
    if (config[dataSource]) {
        const dataSourceYaml = processTemplate(__dirname + `/templates/${dataSource}.datasource.yaml`, config[dataSource]);
        fs.writeFileSync(__dirname + `/generated/${dataSource}.yaml`, dataSourceYaml);
    }
});

let datasources = [];
['erc20', 'erc721', 'erc404'].forEach((dataSource) => {
    if (config[dataSource]) {
        const dataSourceYaml = yaml.load(fs.readFileSync(__dirname + `/generated/${dataSource}.yaml`, 'utf8'));
        datasources.push(dataSourceYaml);
    }
});

let subgraphYaml = processTemplate(__dirname + '/templates/subgraph.template.yaml', { datasources });

// // Write the final subgraph.yaml file
fs.writeFileSync(__dirname + '/../subgraphs/subgraph.yaml', subgraphYaml);

console.log('subgraph.yaml generated successfully.');