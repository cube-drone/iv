const { task, desc } = require('jake');
let { run } = require('./automations/run')

desc('List all of the things iv can do');
task('default', async function(){
    await run('jake -t')
})

desc('Boot up all the backing services');
const dockerup = async () => {
    await run('docker compose up -d')
}
task('start', dockerup);
