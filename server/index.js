const express = require('express');
const path = require('path');
const execFile = require('child_process').execFile;

async function spawnChildProcess( fileName, year, category ) {
   const startTime = process.hrtime.bigint();
   console.log( `spawnChildProcess() started` );

   // Spawn child process to update JSON file, don't return until it's done
   const result = await new Promise( ( resolve, reject ) => {
      execFile( fileName, [ year, category ], function(err, data) {
         if ( err ) {
            reject( err );
         }
         else {
            console.log( `nflstatsrequest.exe completed with return ${data.toString()}` );
            resolve( data );
         }
      });
   });

   const elapsedTime = process.hrtime.bigint() - startTime;
   console.log( `spawnChildProcess() finished in ${elapsedTime} microseconds` );
   return result;
}

const PORT = process.env.PORT || 3006;
const app = express();

app.get('/api/:year/:category', async (req, res) => {
   // Spawn child process and wait for execution to finish
   await spawnChildProcess( 'request.exe', req.params.year, req.params.category );

   // Now that JSON is updated, send it to the client
   res.sendFile(path.resolve(__dirname, 'stats_table.json'));
});

app.use(express.static(path.resolve(__dirname, '..', 'public')));

app.listen(PORT, () => {
   console.log(`Server is listening on port ${PORT}`);
});