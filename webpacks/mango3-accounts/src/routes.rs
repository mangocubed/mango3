use mango3_web_utils::prelude::*;

use crate::components::Layout;
use crate::pages::LoginPage;

#[derive(Clone, Routable)]
#[rustfmt::skip]
pub enum Routes {
    #[layout(Layout)]
        #[route("/login")]
        LoginPage {},
}
