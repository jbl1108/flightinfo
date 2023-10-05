use sailfish::TemplateOnce;
use crate::model::flight::Flight;

#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
pub struct Home {
    pub flights : Vec<Flight>
}