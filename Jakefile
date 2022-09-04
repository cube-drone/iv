const { task, desc } = require('jake');
let { run } = require('./automations/run')

desc('List all of the things iv can do');
task('default', async function(){
    await run('jake -t')
})

desc('Boot up all the backing services');
const dockerup = async () => {
    await run('docker compose up -d')
    let redisPrimeUrl = 'redis://four@127.0.0.1:41001';
    let redisLocalUrl = 'redis://four@127.0.0.1:41002';

    await run('cd api && cargo run', {
        IV_REDIS_LOCAL_URL: redisLocalUrl,
        IV_REDIS_PRIME_URL: redisPrimeUrl,
        IV_BIND: '0.0.0.0:3000',
    });
}
task('start', dockerup);
