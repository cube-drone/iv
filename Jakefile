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
    let Redis = require('ioredis');
    try{
        let cluster = new Redis.Cluster([
            {
                port: 41001,
                host: '127.0.0.1'
            },
            {
                port: 41002,
                host: '127.0.0.1'
            },
            {
                port: 41003,
                host: '127.0.0.1'
            },
        ]);

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