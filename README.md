# web-actix-rust

```SQL
create table teacher (
       id serial primary key,
       name varchar(140) not null,
       picture_url varchar(100) not null,
       profile varchar(100) not null
);

必须设置数据库字段是 not null 否则会出现 mismatch type 的错误，expected String，但是 found option enum（可为空）
或者 设置字段为 pub description: Option<String> 
```