## 📙 `Backend-bluePi-Assignment` project guide

นี่เป็น project rust project แรกของผมนะคับ โครงสร้าง project เลยจะเหมือนของ go หน่อย ผมเขียน unit test ไม่ทัน ;-; 

### 📍 Dependencies
- ✅ rust v.1.83
- ✅ mysql v.9.1
- ✅ diesel_cli (ลงผ่าน command `cargo install diesel_cli`)
- 📄 หรือสามารถรัน project ผ่าน docker-compose ได้ครับ 

### 📁 Setup project with docker
- run project with docker
```bash
    1: 📄 docker-compose up --build -d
```
- 💡 path api backend http://localhost:8080 สามารถดู api spec ผ่านไฟล์ [postman](/postman_collection.json)

### 📁 Setup project for development
- Install nvm
```bash
    1: 📄 brew install nvm
    2: 📄 export NVM_DIR="$HOME/.nvm"
  [ -s "/opt/homebrew/opt/nvm/nvm.sh" ] && \. "/opt/homebrew/opt/nvm/nvm.sh"
  [ -s "/opt/homebrew/opt/nvm/etc/bash_completion.d/nvm" ] && \. "/opt/homebrew/opt/nvm/etc/bash_completion.d/nvm"  
```
- Install node
```bash
    1: 📄 nvm install v18.16.1
    2: 📄 nvm alias default 18.16.1
```
- Check node version
```bash
    1: 📄 node -v # v.18.16.1
    2: 📄 npm -v  # v.9.5.1
```
- Clone repo นี้แล้ว cd มาที่ project
- Install node dependencies
```bash
    1: 📄 npm install
```
- Migrete database
```bash
    1: 📄 npm run migrete
```
- Run project
```bash
    1: 📄 npm run dev
```
- 💡 เข้าไปที่ http://localhost:8080 จะเจอกับข้อความ `Hello world`
