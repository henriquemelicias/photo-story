use std::sync::LazyLock;

use leptos::{logging, Serializable};
use reqwest::Client;

#[cfg( feature = "hydrate" )]
use {
    rkyv::{
        de::deserializers::SharedDeserializeMap, from_bytes, ser::serializers::AllocSerializer, to_bytes,
        validation::validators::DefaultValidator, Archive, Deserialize, Serialize,CheckBytes
    },
    futures::future,
};

pub static CLIENT: LazyLock<reqwest::Client> = LazyLock::new( || {
    Client::builder()
        .pool_max_idle_per_host( 100 )
        .http2_prior_knowledge()
        .build()
        .expect( "Failed to build reqwest client" )
} );

#[cfg( not( feature = "hydrate" ) )]
use js_sys::Uint8Array;

#[cfg( not( feature = "hydrate" ) )]
pub async fn fetch<T, K, const N: usize>( path: &str, body: K ) -> Option<T>
where
    T: Serialize<AllocSerializer<1024>>,
    T: Serializable + Archive,
    T::Archived: for<'b> CheckBytes<DefaultValidator<'b>> + Deserialize<T, SharedDeserializeMap>,
    K: Serialize<AllocSerializer<N>>,
{
    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map( |a| a.signal() );

    let bytes = gloo::net::http::Request::post( path )
        .abort_signal( abort_signal.as_ref() )
        .body( Uint8Array::from( to_bytes( &body ).ok()?.as_slice() ) )
        .send()
        .await
        //.map_err( |e| log::error!( "{e}" ) )
        .ok()?
        .binary()
        .await
        .ok()?;

    // abort in-flight requests if the Scope is disposed
    // i.e., if we've navigated away from this page
    leptos::on_cleanup( move || {
        if let Some( abort_controller ) = abort_controller {
            abort_controller.abort()
        }
    } );
    from_bytes::<T>( &bytes ).ok()
}

#[cfg( feature = "hydrate" )]
pub async fn fetch<T, K, const N: usize>( path: &str, body: K ) -> Option<T>
where
    T: Serialize<AllocSerializer<1024>>,
    T: Serializable + Archive,
    T::Archived: for<'b> CheckBytes<DefaultValidator<'b>> + Deserialize<T, SharedDeserializeMap>,
    K: Serialize<AllocSerializer<N>>,
{
    let start = std::time::Instant::now();

    let request = CLIENT
        .get( path )
        .body( to_bytes( &body ).ok()?.to_vec() )
        .send();

    let (abortable_request, abort_handler) = future::abortable( request );

    let bytes = abortable_request
        .await
        // TODO: REMOVE OR CHANGE THE LOGS TO WARN
        .map_err( |e| logging::error!( "{e}" ) )
        .ok()?
        .map_err( |e| logging::error!( "{e}" ) )
        .ok()?
        .bytes()
        .await
        .ok()?;

    logging::log!( "fetch took {:?}", start.elapsed() );

    leptos::on_cleanup( move || {
        abort_handler.abort();
    } );

    from_bytes::<T>( &bytes ).ok()
}
