# C4 Model of The Current Architecture

Context Diagram
![Alt text](c4/image-1.png)
![Alt text](c4/image-2.png)

Container Diagram
![Alt text](c4/image-3.png)

Deployment Diagram
![Alt text](c4/image-4.png)

# Risk Storming
Hasil dikusi antara Juan Dharmananda Khusuma dan Maurice Yang dari kelompok C15

## Future Architecture
![Alt text](c4/image-1.png)
![Alt text](c4/image-2.png)
![Alt text](c4/image-8.png)

## Identification
![Alt text](c4/image-5.png)
Risk matrix

![Alt text](c4/image-6.png)

![Alt text](c4/image-7.png)

## Consenssus

1. Dua partisipan mendapati bahwa masalah keamanan kredensial database merupakan risiko yang paling tinggi namun kemungkinan terjadinya kecil (3).

2. Dua partisipan mendapati terdapat kesulitan dalam manajemen migrasi schema database akibat arsitektur microservice, namun seluruh microservice hanya menggunakan satu database (6).

## Mitigations
Arsitektur diperbaharui agar setiap microservice menggunakan database tersendiri yangt terisolasi dari database microservice lainya.
Kelebihan:
- Mempermudah manajemen migrasi schema database
- Meminimalisir risiko keamanan kredensial database

Kekurangan:
- Memerlukan biaya tambahan untuk pengelolaan database
- Perlunya pengecekan referential integrity antar database

# Individu 
Component Diagram
![Alt text](c4/image-9.png)

Code Diagram
![Alt text](c4/image-10.png)