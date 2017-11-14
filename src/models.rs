extern crate chrono;

use super::schema::users;
use super::schema::articles;
use super::schema::tags;
use super::schema::articletags;

use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    //pub following: Option<bool>
}

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
#[allow(non_snake_case)]
pub struct AdvancedArticle {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub createdAt: NaiveDateTime,
    pub updatedAt: Option<NaiveDateTime>,
    pub author: i32,
    //pub favorited: bool,
    //pub favoritesCount: i32,
    pub tagList: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[table_name = "articles"]
#[derive(AsChangeset)]
#[derive(Identifiable)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct UpdatedArticle<'a> {
    pub id: i32,
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub author: i32,
    pub createdat: NaiveDateTime,
    pub updatedat: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "users"]
#[derive(Debug)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub token: &'a str,
    pub username: &'a str,
}

#[derive(Insertable)]
#[table_name = "articles"]
#[derive(Debug)]
pub struct NewArticle<'a> {
    pub title: &'a str,
    pub slug: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub author: i32,
    pub createdat: NaiveDateTime,
    pub updatedat: Option<NaiveDateTime>,
    //pub tagList: &'a Vec<str>,
}

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct IncomingArticle {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tagList: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct IncomingArticleResult {
    pub article: IncomingArticle,
}

#[derive(Identifiable, Queryable, Associations)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[has_many(articletags)]
#[allow(non_snake_case)]
pub struct Article {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub createdAt: NaiveDateTime,
    pub updatedAt: Option<NaiveDateTime>,
    pub author: i32,
    // pub favorited: bool,
    // pub favoritesCount: i32,
    // pub tagList: Vec<String>,
}

#[derive(Identifiable, Queryable, Associations)]
#[derive(Serialize, Deserialize)]
#[table_name = "articletags"]
#[primary_key(id)]
#[belongs_to(Article, foreign_key = "articleid")]
#[belongs_to(Tag, foreign_key = "tagid")]
pub struct ArticleTag {
    pub id: i32,
    pub articleid: i32,
    pub tagid: i32,
}

#[derive(Insertable)]
#[table_name="articletags"]
pub struct NewArticleTag {
    pub articleid: i32,
    pub tagid: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[has_many(articletags)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Tag {
    pub id: i32,
    pub tag: String,
}
