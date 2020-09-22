const { myWatermark } = require('../pkg/ssvm_nodejs_starter_lib.js')

const http = require('http')
const url = require('url')
const multer = require('multer')
const upload = multer({
  storage: multer.memoryStorage()
}).single('file')
const hostname = '0.0.0.0'
const port = 3000;

const server = http.createServer((req, res) => {
  if(req.url == '/upload' && req.method.toLowerCase() === 'post'){
    console.log(JSON.stringify(console.log(req.body)))
    upload(req,res,(err) => {
      if(err instanceof multer.MulterError){
        console.log(err)
        res.writeHead(200, {'Content-Type': 'application/json'})
        res.end('Fail')  

      }else{
        const img_base64 = myWatermark("©Copyright by SSbal",req.file.buffer)
        res.writeHead(200, {'Content-Type': 'image/jpg'})  // return images
        res.end(img_base64,'Base64');
      }
    })
  }
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
