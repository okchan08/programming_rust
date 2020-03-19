extern crate iron;
#[macro_use] extern crate mime;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;


fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
            <title>GCD calculator</title>
            <form action="/gcd" method="post">
              <input type="text" name="n"/>
              <input type="text" name="n"/>
              <button type="submit">Compute GCD</button>
            </form>
            "#);
    Ok(response)
}

use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Erro parsing data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(
                        format!("Value fo 'n' parameter not a number: {:?}\n", unparsed));
                return Ok(response);
            }
            Ok(n) => {numbers.push(n);}
        }
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
            format!("GCD of {:?} is <b>{}</b>\n",
                    numbers, d));
    Ok(response)
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            let t = n;
            n = m;
            m = t;
        }
        m = m % n;
    }
    n
}
