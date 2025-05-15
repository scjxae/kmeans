 // #![allow(warnings)]

mod kmeans;
mod misc;

use kmeans::kmeans;
use misc::Point;
use csv::Reader;

const PARAMETEREY_KUTAS: [(i32, bool); 10] = [
    (2i32, false),
    (3i32, false),
    (4i32, false),
    (5i32, false),
    (7i32, false),
    (2i32, true),
    (3i32, true),
    (4i32, true),
    (5i32, true),
    (7i32, true)
];

//jak chcesz gifa to w misc.rs masz
fn main() {
    let k = 10;
    let mahal = false;
    let gifname = "autos";
    let path = format!("{}", "autos.csv");
    // tworze nowy wektor przetrzymujacy typ Point
    // czyli tuple (float, float) oraz wczytuje dane
    //             (  f64,   f64)
    let mut points = Vec::<Point>::new();
    let mut data = Reader::from_path(path).expect("blad odczytu");
    let records = data.records();
        
    // odczytuje kazdy wiersz
    // city-mpg - 23
    // highway-mpg - 24
    // price - 25
    for r in records {
        // i dane z wybranych kolumn
        let r           = r.expect("Blad przy odczytywaniu rekordu!");
        let city_mpg    = r[23].trim().to_string();
        let highway_mpg = r[24].trim().to_string();
        let price       = r[25].trim().to_string();

        if price.is_empty() {continue;}

        println!("price: {}, city-mpg: {}, highway-mpg: {}",
                 price, city_mpg, highway_mpg);
        
        // sczytane dane konwertuje na floaty
        let y  = price.parse::<f64>().unwrap();
        let x1 = highway_mpg.parse::<f64>().unwrap();
        let x2 = city_mpg.parse::<f64>().unwrap();
        let p  = Point{x: x1 + x2, y}; // tworze punkty
        points.push(p); // i wrzucam do wektora
    }
    
    // biore nazwy kolumn
    let headers = data.headers();
    let a = headers.as_deref().unwrap().get(25).unwrap().trim();
    let b = headers.as_deref().unwrap().get(23).unwrap().trim();

    //formatuje nazwe pliku w zaleznosci od parametrow
    for p in PARAMETEREY_KUTAS {
        let k = p.0;
        let mahal = p.1;
        let gifname = {
            match mahal {
                true  => {format!("{}_{}_{}_{}_mahal.gif", gifname, k, a, b)},
                false => {format!("{}_{}_{}_{}_eud.gif"  , gifname, k, a, b)},
            }
        };

    //kmeans przyjmuje parametry
    //  Vec<Point>, i32,  bool,  &String, String
    kmeans(&points,   k, mahal, &gifname, "kmeans".to_string());
    }
    // gdzie & oznacza przekazanie zmiennej poprzez referencje
    
}

#[allow(dead_code)]
fn save_points(x: &Vec<Point>, path: &str) -> Result<(), std::io::Error> {

    let mut outstr = String::new();
    for p in x {
        outstr.push_str(format!("{},{};", p.x, p.y).as_str());
    }
    std::fs::write(path, outstr)
}
    // points = make_points();


    // //to wtedy czytamy
    // if !saving_points {
    //     let temp = std::fs::read_to_string(&datapath).unwrap();

    //     for p in temp.split_terminator(";") {
    //         let (x, y) = p.split_once(",").unwrap();
    //         // println!("x: {}, y: {}", x, y);
    //         let x = x.parse::<f64>().unwrap();
    //         let y = y.parse::<f64>().unwrap();
    //         points.push(Point(x, y));
    //     };
    // }
    
    // if saving_points {
    //     points = make_points();
    //     save_points(&points, &datapath).expect("failed to write");
    // }
