use rand::{Rng, rand_core::le};
#[warn(unused_must_use)]
use rcon::Connection;
use std::time::Duration;
use std::fs;
use std::io;

async fn mcsort(a: Vec<u16>) -> Vec<u16>{
    let address = "127.0.0.1:25575";
    let password = &fs::read_to_string("rcon_password").unwrap();
    let mut conn = Connection::connect(address, password).await.unwrap();

    let maxvalue = *a.iter().max().unwrap();
    let length = a.len();


    conn.cmd(&format!("fill {} {} 0 {} 319 0 air", -2, -1, (maxvalue+1) as i32)).await.unwrap();

    std::thread::sleep(Duration::from_secs(1));

    conn.cmd(&format!("setblock -1 -1 0 stone")).await.unwrap();
    conn.cmd(&format!("setblock {} -1 0 stone", maxvalue))
        .await
        .unwrap();
    for i in 0..maxvalue {
        conn.cmd(&format!("setblock {} -1 0 stone", i))
            .await
            .unwrap();
    }
    for i in 0..length {
        conn.cmd(&format!("setblock -1 {} 0 stone", i))
            .await
            .unwrap();
        conn.cmd(&format!("setblock {} {} 0 stone", maxvalue, i))
            .await
            .unwrap();
    }

    conn.cmd(&format!("tick freeze")).await.unwrap();
    for i in 0..length {
        let cv = a[i];
        /*for j in 0..cv {
            conn.cmd(&format!("setblock {} {} 0 sand", j, i))
                .await
                .unwrap();
        }*/
        if cv == 0 {
            continue;
        }
        conn.cmd(&format!("fill 0 {} 0 {} {} 0 sand", i, cv-1, i)).await.unwrap();
    }
    conn.cmd(&format!("setblock -2 -1 0 stone")).await.unwrap();
    conn.cmd(&format!("setblock -2 {} 0 sand", length + 1))
        .await
        .unwrap();

    conn.cmd(&format!("tick freeze")).await.unwrap();
    loop {
        let rst = conn
            .cmd(&format!("execute if block -2 0 0 minecraft:sand"))
            .await
            .unwrap();
        if rst == "Test passed" {
            break;
        }
    }
    println!("sorted");

    let mut sorted: Vec<u16> = vec![0; length];
    let mut last = maxvalue;
    for i in 0..length {
        /*for j in 0..maxvalue {
            let rst = conn
                .cmd(&format!("execute if block {} {} 0 minecraft:sand", j, i))
                .await
                .unwrap();
            println!("{}",rst);
            if rst == "Test passed" {
                sorted[i] += 1;
            } else if rst == "Test failed" {
                break;
            }
            
            //std::thread::sleep(Duration::from_millis(10));
        }*/
        let mut left: u16 = 0;
        let mut right = last;
        while left < right {
            let mid = (left + right) / 2;
            let rst = conn
                .cmd(&format!("execute if block {} {} 0 minecraft:sand", mid, i))
                .await
                .unwrap();
            if rst == "Test passed" {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        sorted[i] = left;
        last = left;
        conn.cmd(&format!("setblock -1 {} 0 minecraft:diamond_block", i)).await.unwrap();
    }
    sorted.reverse();
    return sorted;
    //return vec![0];
}

#[tokio::main]
async fn main() {
    let mut rng = rand::rng();
    let mut a = Vec::new();
    for _ in 0..150 {
        a.push(rng.random_range(0..150));
    }
    //let mut a = vec![0,1,2,3,4,5,6,7,8,9];
    let mcs = mcsort(a.clone()).await;
    a.sort();


    if a == mcs {
        println!("=")
    }
    println!("{:?}, {:?}", mcs, a);
}
