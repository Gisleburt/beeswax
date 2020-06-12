use crate::resource::*;

pub trait FromAnyResource {
    fn from_any_resource(r: &AnyResource) -> Option<&Self>;
}

macro_rules! any_resource {
    ( $( $i:ident ),* ) => {
        #[derive(Clone, Debug)]
        pub enum AnyResource {
            $( $i($i), )*
        }

        $(
            impl FromAnyResource for $i {
                fn from_any_resource(ar: &AnyResource) -> Option<&$i> {
                    match ar {
                        AnyResource::$i(r) => Some(r),
                        _ => None,
                    }
                }
            }

            impl From<$i> for AnyResource {
                fn from(r: $i) -> Self {
                    AnyResource::$i(r)
                }
            }
        )*
    };

}

any_resource!(
    AccountAlert,
    Advertiser,
    Authenticate,
    Campaign,
    Creative,
    CreativeLineItem,
    LineItem,
    View,
    ViewList
);
