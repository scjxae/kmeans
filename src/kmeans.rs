


use nalgebra::Matrix2;
use num::Zero;

use crate::misc::{*, plot_gif};
use std::cmp::Ordering;
use nalgebra::DVector;
use plotters::backend::BitMapBackend;
use crate::misc::get_dem_groups;

type P = Vec<Point>;

pub fn invcov(points: &P) -> Matrix2<f64> {

    let n = points.len();

    //obliczam srednia dla x i y
    let mx = points.iter().map(|x| x.x).sum::<f64>() / n as f64;
    let my = points.iter().map(|y| y.y).sum::<f64>() / n as f64;

    let mut cov = Matrix2::zeros();

    for point in points {
        //tera roznice i tworze wiersz
        let diff = DVector::from_vec(vec![point.x - mx, point.y - my]);
        //sumuje i dodaje wiersz do macierzy
        cov += &diff * diff.transpose();
    }

    //ostatni krok
    let out = cov / (n as f64 - 1.);

    //zartowalem, jeszcze odwrocic trzeba
    out.try_inverse().unwrap()
}

pub fn cluster_quality((x, centroids): (&P, &P), xgroups: &Vec<usize>, mahal: bool, invcov: &Matrix2<f64>) -> f64 {
    
    let mut lens = vec![0f64; centroids.len()];
    let mut denum = 0f64;
    let mut num = 0f64;
    
    let mut dummy = Matrix2::<f64>::zeros();

    let d: fn(&Point, &Point, &Matrix2<f64>) -> f64 = {
        if mahal {dummy = *invcov; mahald}
        else {eud}
    };

    // obliczam mianownik i przy okazji
    // liczbe elementow w danej grupie
    for (xi, group) in x.iter().zip(xgroups) {
        lens[*group] += 1.;
        denum += d(&xi, &centroids[*group], &dummy);
    }

    // tera licznik
    for (ci, ca) in centroids.iter().enumerate() {
        // ze wzoru
        for cb in centroids.iter().skip(ci + 1) {
            let dist = d(&ca, &cb, &dummy);
            // druga suma we wzore powtarza sume dla kazdego
            // elementu w grupie AKA: rozmiar grupy
            num += dist * lens[ci]; 
        }
    }

    // we done
    num / denum
}

pub fn kmeans(x: &P, k: i32, use_mahalanobis: bool, path: &String, title: String) -> (Vec::<Point>, Vec::<Vec::<i32>>) {
    
    // inicjalizuje odpowiednio wektory P i C,
    let mut duze_p    = Vec::<Vec::<i32>>::new();
    let mut centroids = Vec::<Point>::new();

    // reszte przydatnych zmiennych
    let mut dists = vec![0.; k as usize];
    let mut counter = 0;
    let mut converged = false;
    let mut groups: Vec<usize> = vec![];
    let mut quality: f64 = 0.;

    // oraz pole do rysowania i granice wartosci
    let mut drawing_area = BitMapBackend::new("$", (WIDTH, HEIGHT)).into_drawing_area();
    let min_max = get_min_max(x);
    if MAKE_GIF {
    drawing_area = BitMapBackend::gif(path, (WIDTH, HEIGHT), 3).unwrap().into_drawing_area();
    make_plot_area(x, &drawing_area, k, use_mahalanobis, &min_max, &title);
    }

    // wypelniam wektor C losowymi punktami w przedziale min i max
    for _ in 0..k {
        centroids.push(makerand_range(min_max.x().into(), min_max.y().into()));
    }
    
    // wektor P trzymajacy len(x) wektorow z typem int o dlugosci k  
    for _ in 0..x.len() {
        let mut pi = Vec::<i32>::new();
        pi.resize(k as usize, 0);
        duze_p.push(pi);
    }

    // obliczam macierz kowariancji
    // i przypisuje funkcje odleglosci do zmiennej d
    let x_inv: Matrix2<f64> = {
        if use_mahalanobis {invcov(x)}
        // jezeli nie wykorzystuje mahalanobisa
        else {Default::default()} // to przypisuje objekt z domyslnymi danymi
    };

    // d to funkcja przyjmujaca 2 punkty oraz macierz kowariancji
    // w przypadku euklidesa, trzeci argument nie ma znaczenia
    //             x       y           V^-1  zwraca float
    let d: fn(&Point, &Point, &Matrix2<f64>)   ->   f64 = {
        if use_mahalanobis {mahald}
        else {eud}
    };
        
    loop {
        // wektor duze_p ma same zera wiec klonujac go
        // przypisuje wyzerowany wektor z gotowym rozmiarem 
        let mut big_p = duze_p.clone();

        // gdy stwierdzono ze roznice pomiedzy iteracjami
        // sa bardzo male, wypisuje label i przetrzymuje
        // jedna klatke przez chwile
        if converged {
            let texts: Vec<String> = vec![
                format!("F(C) = {:2.3}", quality),
                format!("n = {} done", counter)
            ];
            if !MAKE_GIF {
                print!("{} \t {} \n", texts[0], texts[1]);
                break;
            }
            for _ in 0..15 {
                plot_gif(x, &groups, &drawing_area, &texts);
            }
            break;
        }

        // dla kazdego x i p w zbiorze punktow X i przynaleznosci P
        for (xi, p) in x.iter().zip(&mut big_p) {
            //obliczam odlegosc miedzy punktem x i kazdym srodkiem
            for (c, dist) in centroids.iter().zip(&mut dists) {
                *dist = d(xi, c, &x_inv);
                //zdarzalo sie to sprawdzam czy tak nie jest
                if dist.is_nan() { panic!("dist is inf") }
            }

            let mut minarg = 0;
            let mut min = std::f64::MAX;

            // szukam srodka do ktorego x ma najkrotsza droge 
            for (k, dist) in dists.iter().enumerate() {
                match dist.partial_cmp(&min) {
                    Some(Ordering::Less) => {
                        min    = *dist;
                        minarg = k
                    },
                    _ => ()
                }
            }

            // przypisuje do ktorej grupy nalezy punkt x
            p[minarg] = 1;
        }

        // groups zawiera indeksy do wektora centroids
        // ktory adekwatnie odpowiada kazdemu x'owi
        groups = get_dem_groups(&big_p);
        quality = cluster_quality((x, &centroids), &groups, use_mahalanobis, &x_inv);
        let texts: Vec<String> = vec![
            format!("F(C) = {:2.3}", quality),
            format!("n = {}", counter)
        ];
        plot_gif(x, &groups, &drawing_area, &texts);

        // licznik ile razy roznica pomiedzy poprzednim a nowym 
        let mut converge_count = 0;

        // obliczam nowe centra
        for c in 0..centroids.len() {
            
            let mut sumka = Point::default();
            // suma pik w mianowniku musi byc na poczatku
            // rowna 1, aby nie wystapilo dzielenie przez 0
            let mut len   = 1f64;

            // przechodze przez elementy p i x jednoczesnie
            for p in big_p.iter().zip(x) {
                //sprawdzam czy dany x nalezy do srodka c
                if p.0[c].is_zero() { continue; }
                //jezeli tak to kontynuuje obliczanie
                sumka.x += p.1.x;
                sumka.y += p.1.y;
                len += 1.;
            };

            let mean: Point =
                (sumka.x / len as f64,
                 sumka.y / len as f64).into();

            //sprawdzam czy wartosci znacznie
            //roznia sie pomiedzy krokami
            let converge1 = f64::abs(centroids[c].x - mean.x);
            let converge2 = f64::abs(centroids[c].y - mean.y);
            if (converge1 <= 10e-9) || (converge2 <= 10e-9) {
                converge_count += 1;
                if converge_count == centroids.len() {
                    duze_p = big_p.clone();
                    converged = true;
                    MAKE_GIF.then(|| println!("CONVERGED!!!")); //essa
                }
            }
            
            //przypisuje c nowy srodek
            centroids[c] = mean; 
            
        }
        counter += 1;
    }

    (centroids, duze_p)
} 
