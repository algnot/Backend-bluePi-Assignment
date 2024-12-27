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
    1: 📄 docker compose build
    2: 📄 docker compose up -d
```
- 💡 Path API backend http://localhost:8080 ถ้าไม่แน่ใจว่า server รันจริงไหมลองเข้า http://localhost:8080/product/list ก็ได้คับ ;-; สามารถดู API spec ผ่านไฟล์ [postman](/postman_collection.json)
- 💡 เข้าดู database ผ่าน phpmyadmin ที่ http://localhost:8090 (username: root, password:root)

### 📁 Project Structure

```bash
    - src
        - common #สำหรับเก็บ common function  
        - di #สำหรับจัดการ dependencies ต่าง ๆ เช่น db, server
        - repository #สำหรับกำหนด model และส่วนที่ติดต่อกับ db ทั้งหมด
        - service #ส่วนที่จะเอาออกไปให้ external เรียกใช้
            - http #เวลาเพิ่ม API เส้นใหม่จะเพิ่มใน folder นี้
                - router.rs #สำหรับกำหนด router ของเส้น API ทั้งหมด
            - migrator.rs #เอาไว้สำหรับ migrate starter data ใน db 
        - main.rs #ไฟล์หลักของ project
```
- ตัวอย่างเช่นถ้าเราต้องการเพิ่ม api เส้น `product/list` เราจะไปเพิ่ม folder `/product` ที่ `service/http/product` และไฟล์ที่ชื่อ `list_product.rs` ข้างในและกำหนด router ใน `product.rs`

```rust
// 📄 product.rs
    use actix_web::web;
    // import ด้วย
    use crate::service::http::product::list_product::list_product;

    pub fn product_router(cfg: &mut web::ServiceConfig) {
        // เพิ่ม `/list` logic ทั้งหมดจะอยู่ใน list_product.rs (ยกเว้นส่วนที่ต่อกับ db จะทำผ่าน repository)
        cfg.route("/list", web::get().to(list_product));
    }
```
- หลักจากกำหนด router แล้วถ้าเป็นการเพิ่ม API ครั้งแรกจะต้องนำ `product_router` ไปกำหนดที่ `service/http/router.rs` อีกที
```rust
// 📄 router.rs
    use actix_web::{web};
    // import ด้วย
    use crate::service::http::product::product::product_router;

    pub fn main_router(cfg: &mut web::ServiceConfig) {
        // เพิ่ม product_router ใน `/product`
        cfg.service(web::scope("/product").configure(product_router));
    }
```

