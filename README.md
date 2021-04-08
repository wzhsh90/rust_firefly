<span style="color: red">  rust_firefly 开发脚手架，使用ymal 作为sql文件 </span>

# 基于Rust的后台管理系统，适合新手学习，简单，清爽

## 功能特点

#### 前端基于layui,juicer前端模板引擎。

Layui: https://www.layui.com/demo/

juicer: https://github.com/PaulGuo/Juicer

#### 后端基于actix-web开发。

Actix框架: https://actix.rs/

#### MVC 设计模式,快速入门,方便上手。

#### handlebars 模板引擎，服务端html 渲染更简单。

handlebars: https://github.com/sunng87/handlebars-rust

#### rbatis + ymal 操作数据库，结构更简单清晰, 远离xml 方式，将sql 放在程序代码外，调整后编译不花时间

rbatis：https://github.com/rbatis/rbatis

#### 基于Rust语言特性,有性能、安全保证

## 二次开发 & 技术交流

#### 扫码备注: 'firefly',

![avatar](/static/img/qr.jpg)

## 环境要求

rust: 1.40+ / Mysql: 5.6+

## 目录说明

#### /resource 用于系统默认的配置文件

#### /resource/mybatis 用于存储 sql ymal 文件，支持yaml1.2 并增加yaml 点位标识

#### /src rust源代码

#### /static 用于存储前端css/js/img

#### /templates 模板文件

## 界面载图

#### 登录界面

![avatar](/static/img/login.png)

#### 后台管理

![avatar](/static/img/home.png)

## 使用说明

#### 下载代码

```bash
git clone https://github.com/wzhsh90/rust_firefly.git
cd rust_firefly
```

#### 示例sql数据表

```sql
CREATE TABLE `sys_company_t`
(
    `id`       char(24) NOT NULL,
    `com_name` varchar(100) DEFAULT NULL,
    `com_desc` varchar(100) DEFAULT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;


```

#### 示例ymal 写sql

```yaml
table_name:
  sys_company_t
base_sql:
  id,com_name,com_desc
exist_name:
  select count(*) from ${table_name} where com_name=#{name}
update:
  update ${table_name} set com_name=#{com_name},com_desc=#{com_desc} where id=#{id}
list: |
    select ${base_sql} from ${table_name}
    if name != '':
    where com_name like #{name}

```

***** * 默认用户/名称: FireFly / firefly

#### 运行程序

```bash
默认运行开发模式
cargo run= cargo run dev
cargo run #开发模式: cargo run dev 
cargo run #生产模式: cargo run prod
```

#### 作为一名rust新手，才刚接触rust,开发遇到了不少问题，感谢 rbatis 作者给予帮助，才使得firefly得以完成。

