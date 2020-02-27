use crate::models::*;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
pub struct IndexView {
    pub memos: Vec<Memo>,
}

#[derive(Template)]
#[template(path = "new.html")]
pub struct NewView;

#[derive(Template)]
#[template(path = "edit.html")]
pub struct EditView {
    pub memo: Memo,
}
