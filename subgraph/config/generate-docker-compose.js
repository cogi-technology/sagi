const fs = require('fs');
const yaml = require('js-yaml');
const Handlebars = require('handlebars');

// Helper function to process a YAML template
function processTemplate(templateFile, context) {
    const templateSource = fs.readFileSync(templateFile, 'utf8');
    const template = Handlebars.compile(templateSource);
    return template(context);
}

function main() {
    const configs = yaml.load(fs.readFileSync(__dirname + `/config.yaml`, 'utf8'))['ipfs'];
    const internalIpfsTemplate = __dirname + "/templates/docker-compose-internal-ipfs.template.yaml"
    const externalIpfsTemplate = __dirname + "/templates/docker-compose-external-ipfs.template.yaml"

    // Determine which template to use based on the IPFS configuration
    const selectedTemplate = configs['host'] === 'ipfs' ? internalIpfsTemplate : externalIpfsTemplate;
    const ret = processTemplate(selectedTemplate, configs);
    fs.writeFileSync(__dirname + `/../docker-compose.yml`, ret);
    console.log(`docker-compose.yaml file created successfully.`);
}

main()