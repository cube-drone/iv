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
task('dockerup', dockerup);

desc('Test backing services for awakeness');
const test_connect = async () => {
    let {createCluster} = require('redis');
    try{
        let ports = [41000, 41001, 41002, 41003, 41004, 41005];
        //let ports = [123, 234, 345];

        const cluster = createCluster({
            rootNodes: ports.map(port => {return {
                url: `redis://:bitnami@127.0.0.1:${port}`,
            }}),
        });
        
        cluster.on('error', (err) => {
            console.error(err);
            throw err;
        });
        
        await cluster.connect();

        await cluster.set("test", "ahoy");
        let result = await cluster.get("test");

        console.log(`Ahoy ${result}`);
    }
    catch(err){
        console.error(err);
        return false;
    }
}
task('test_connect', test_connect);