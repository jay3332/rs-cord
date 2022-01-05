#[macro_export]
macro_rules! route {
    ($method:ident, $route:literal, $($k:ident=$v:expr,?)*) => {{
        use crate::http::{RequestMethod, Route};
        Route {
            method: RequestMethod::$method,
            route: format!($route, $($k=$v,?)*),
        }
    }};

    ($method:expr, $route:literal, $($k:ident=$v:expr,?)*) => {{
        use crate::http::Route;
        Route {
            method: $method,
            route: format!($route, $($k=$v,?)*),
        }
    }};
}
