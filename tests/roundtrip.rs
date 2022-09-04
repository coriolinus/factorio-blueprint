use factorio_blueprint::{BlueprintCodec, Error};

/// Decodes a blueprint string into the serde_json Value
fn decode_to_json_value(blueprint: &str) -> Result<serde_json::value::Value, Error> {
    let mut out = Err(Error::NoData);
    BlueprintCodec::decode_reader(blueprint.as_bytes(), |reader| {
        out = serde_json::from_reader(reader).map_err(|e| e.into());
        Ok(())
    })?;
    out
}

/// Decodes a blueprint string and compares that parsed json matches
/// after we decode and then re-encode it.
fn roundtrip_blueprint_test(blueprint: &str) {
    let mut w = std::fs::File::create("output1.json").unwrap();
    serde_json::to_writer_pretty(&mut w, &decode_to_json_value(blueprint).unwrap()).unwrap();
    let blueprint_after =
        BlueprintCodec::encode_string(&BlueprintCodec::decode_string(blueprint).unwrap()).unwrap();
    let mut w = std::fs::File::create("output2.json").unwrap();
    serde_json::to_writer_pretty(&mut w, &decode_to_json_value(&blueprint_after).unwrap()).unwrap();

    // Compare using serde_json Value since the json string order is not guaranteed
    assert_eq!(
        decode_to_json_value(blueprint).unwrap(),
        decode_to_json_value(&blueprint_after).unwrap(),
    );
}

#[test]
fn test_roundtrips_empty() {
    // Empty blueprint book
    roundtrip_blueprint_test("0eNqrVkrKKU0tKMrMK4lPys/PVrKqVsosSc1VskJI6IIldJQSk0syy1LjM/NSUiuUrAx0lMpSi4oz8/OUrIwsDE3MLY3MzQyNTUwMDGprAVU7HPM=");
    // Empty blueprint (in a blueprint book, cannot export otherwise)
    roundtrip_blueprint_test("0eNqrVkrKKU0tKMrMK4lPys/PVrKqRogUK1lFI3FBcpklqblKVkhiOkplqUXFmfl5SlZGFoYm5pZG5maGxiYmBga1OkqZeSmpFUpWBrWxOhg6dcHW6SglJpdklqXGw5TiMa8WAEFMONI=");
    // Empty deconstruction planner
    roundtrip_blueprint_test("0eNpdy0EKgCAQAMC/7NlATbL8TIQtIdgaunYR/16XLl0HpsGOPlHhXD2HROsVNyLM4BoUZA50FHBUYxQQGE9wvzB8QcCNubwCTs/K2EXbSY3GSNn7A7BrI8Q=");
    // Empty upgrade planner
    roundtrip_blueprint_test("0eNo1ykEKgCAQBdC7/LWBmmR5mRAaRLBJ1NqId2/lW7+ON4fiLzpz8sxU4DoqtRY5VDh+UxKIjW64OZc5BT4qNT4Mp3dl7KHtplZjpBzjB4YmHaw=");
}

#[test]
fn test_labels_icons_descriptions() {
    // Deconstruction planner with label, desc, icon
    roundtrip_blueprint_test("0eNpNjlEKwyAQRO+y3wZiKk3rVUopVpcgmE3QTWgJ3r1qKfRvZngzzAEO7UKJ42bZL/RYgyHCCPqAhMyeplS1w2SjXysCujkQ4GsT9K2gfiITKsjvFQux+8hbSQSQmWvwJToJufTI4Qu0zPdqGOc2+X+j+90QEMwTyzIwJi52x5jaieEi1XgdxrM8KdX3OX8AwjBFBw==");
    // Upgrade planner with label, desc, icon
    roundtrip_blueprint_test("0eNo1jVEKgzAQRO+y3xHUhtp6gt6hSEnNEgK6hmSVFsndu7Ht387Mm50d1uCisfgIkyHCCP0OCZk9uVRui2mMPrBfCPpDgQI/LiTpXVDvyEwF5HdAITYfeRVHAZm5GF+iukGWHll8Qd/koQjGWfLffvXfVzCZJ8pLYEwscsOYjvX20uju2nbn5qR1Xef8AXXWPwY=");
    // Blueprint book with label, desc, icon
    roundtrip_blueprint_test("0eNpFjUEKgzAQRa9SZh1BrdQ2u55DiiRxKENjIkmUFsndOw1Cl/Pe/3920HbFJZBLo/b+BXIHSjiD/IuqCAFWabQs7ienZmQwYTSBlkTeMS4XUzLeRZDDDpGeTtnfZPosyJGNQlqZCCgL8khUPWTuuQnfIJv8EKBMog3HA9UCNgyxvGmvTdff2v7SnLuurnP+AkCkP98=");
}

#[test]
fn test_simple_entity_tile() {
    // Single chest with bar
    // Single concrete tile
    // Label, desc, icon
    roundtrip_blueprint_test("0eNptkN1uwjAMhd/F1wFRGgrkVdCE+uNtllI3SsxEVeXd5xQ0TYLLY53z2ccLdP6GIRILuAUGTH2kIDQxuFWBAeonTuAuCyT64tYXo8wB1UGCozq4HYtKgug3/Tcmgaw5HvAOrsofBpCFhPCBWcV85dvYYVTDW4CBMCV6XLKAcmxdbw8GZnCN3R6U37UarpuCF/JP9mvoGcl/WyJ+EuOw0Vp9REEohLWJ+/cNA77t0JeraAwedfCDMa3o/amyx/P+2FS1tbtdzr/GoWoB");

    // snap-to-grid, absolute snapping
    roundtrip_blueprint_test("0eNptkF1qwzAQhO+yz1KIE/+kukopRba37oK8FtI61ATdvZIT2hL6OGLnmxndoHcr+kAsYG4Q2Xoti54CjUV/gakVbGCapMD2cXGroC5XnngCI2FFBX6JJLSwDuis0BWfEN2OuGQEDQtHMK85iSa2rhzI5hEMkOAMCtjORUVBdHr4xChQfDxiBlXpTQGy5DS8Y3axvfM69xjywb+A34aPQro5NHsn3R7Kst5m77ktdCH3QD972rujSz8ZAT+IcdR51BBQEApg32H+fKsCZ3t0pRPN3mF+uGKIO/l0qeru5dS11bmuj8eUvgGX/IJu");

    // snap-to-grid, relative
    roundtrip_blueprint_test("0eNptkNFOwzAMRf/Fzwla17JBfmVCKG3NsJS6UeIhqqn/jt31ASFeoji69/je3KFPN8yFWCDcoXLMXmZ/LTTa/A2hc7DouTqgYeYK4aIyunJMJpAlIwQgwQkccJxsqoKY/PCJVcB8PKKCmvXNAbKQED4w27C8823qsajgX4CDPFf1zLwH8u3T85bJLorvo3rbk9GF0o7+63m08FZj31HwgxhHr6WGgoJggK1H+PUnDlLsMVkmmnJCffjCUjfy8aXpzq/H86lpu+5wWNcfHcJrAg==");
}

#[test]
fn test_half_setup_upgrade_planner() {
    roundtrip_blueprint_test("0eNo1jVEKwyAQRO8y3waSVJrUq5RShGyDEDei29Ag3r0K7d+bYXiT8Q5rtAs9w2aZKcJkJBJxvKbG3oZAseI9Q/bWyBkIBsTi5IQCW9/yyybpUticSLUUBccLfWDm8mhByNfR76z7nykcVe52hhnnQU+3cboOF637vpQvrWw0ZA==");
}

#[test]
fn test_decon_with_filters() {
    roundtrip_blueprint_test("0eNplj+8KglAMxd9lnxW0JMtXiZCbLrl03ZVthiG+e5OI/n3aOOc3ds4MLTaRRHls1Eeqh+CIkKGaQVDVUyfrjqRe7/XFB0U25TgDuR6hApwGRpFU2ZEMkTU9Y1BIwFOLE1TZkvyxo1nccbT5Q+cftKhrrqknQbanb2aznBJQH7AWDPiM3ccWV8cgxd6Ov2ulr1oJ3Cy/KQbv86I8bMpdvi2KLFuWB1nMXoA=");
}

#[test]
fn test_trains() {
    // COMMON:
    // Train blueprint
    // With fuel
    // Train wagon has filter and bar

    // No stations on schedule, snap-to-grid relative
    roundtrip_blueprint_test("0eNqNlM1ugzAQhF8l2jOuMD8h4ZZrT71XUWXIhloCG9kmLYry7l1DSlQlqXwCm9lvvMvAGap2wN5I5aA8g1WiZ06zxsiDX39DmWwiGKHk8SUCWWtloXwnoWyUaL3EjT1CCdJhBxEo0fmVEbIFX6AOSAx+2UeAykknca6fFuOHGroKDQmWSkelilmne6L12lKJVtejsHQ+CoEP0mA9P1pHYJ2Y7+EV7WBXb2jQiNXOiHo6x51fsvi1utaddvKE937xZJeSnTaSCFeT+KXIo6lh64W19oPI48sDn3Txsb6x5tOxaTTPWkv/tpY8QGbBSB6KzEORwcR1KDG47WIh1sI0mn2JhqR3vOLfV6ZOtKXN6MVH2To0cxp/Y3obxFFYx6SyaEg0JeiqSW5RHWjHNEbTlVXYOvApr4SPV/EoDJvQoeShQ9mGEotQIo9DkdtgJA+OV3C+eBLMfJKw/fwFE+D2A4zgRImYBRueFdukWPM0y2L6tn8AvZe0mg==");

    // One station on schedule with no wait_conditions
    roundtrip_blueprint_test("0eNqNlU1vwjAMhv8K8rmdmn5Q6G3XnXaf0BSKKZFKUiUpG0L89zktA6YV5FOV5M1jx3njnmDd9thZpT1UJ9igq63qvDIaqmEEEajaaAfVxwmcarRsg9AfOySF8rgnhZb7MLJStXCmDXqD31CJ8yoC1F55heP+YXD81P1+jZYE152eturYedMRrTNOjRmcgDB5WkZwhKrMCL1RFutxcR6B8/KS6hu63s3e0aKVs1cr6yGTfxHTa8TW1GZvvDrgRMQsGSMWFNFYRZBLnOSlLKLh1C5oaxOqUSTniVDZNZQLp2t2Ph7q8/h8xd/zpRPQnA9dsqEFG5oJNnTOh2ZsaHmF1tI2Jv6SDUknkOXz+9MHmjL2GPRb1Xq0oz9/jXuryFY6Hyvt0JJocNRFk97M29OMbayhb7zG1kPw/VoGu5VTzljwS1OwS7PkQ/l2EwmfyvebEGxqzjecSPlUvuME/xnn/MsS/HecP7ot8pird7jp20tzvXW0ME7v1sfe/bxXrgJwaOfV3S8hglaSpWluSDeCAz2VMYGFyMtlWs5FlucJdcAfLFMS4w==");

    // One station on schedule with all wait_condition types, and snap-to-grid relative
    roundtrip_blueprint_test("0eNqVVk2PmzAQ/Ssrn3HFRwgJqir12tPeq1XkwIS1Fmw0NtlGq/z3jjENqZINXuUQDPPefPD8zAfbtwP0KJVl5QczSvTcat6grN36DyuTTcROrFyfIyYrrQwrf1OcbJRoXYQ99cBKJi10LGJKdG6FQrbMAVQNjuL8EjFQVloJHj8uTjs1dHtACrggLUEVN1b3xNZrQxCtpkp4MlaSEXEtESr/aB0xY4W/Zr/ADObpGRBQPP1EUY113ORLL/laXelOW3mE23zpmC6ndBolMUxJ4m9FHo0NGxdYaTeIPD7fyZNd8hjXWPNq+Tiaz1rL/28tvUO5CqUMZsxDGbNQxnUoYx7KWFwYK4GN5u+iodAbvu3DN6aOdEvjyQUfZGsBvRj/qXQexEEYy6UygBQ0CmiKSWelDnQHG9T0z/fQWuZEvhdOXcU9LWxCh1KEDmUbyrgNZUziYHUFyytJgjmDBZakwZyfSIzelaleoR7ayZFmI3Dr9Oq5N7yHFhOxdyHtjuyxHpN7SiLsBcJuskiNFDddW9k5w7GyejPOYuPYdfkAIJWg2o80hBmWLaIOQ+v970EMdD2RLqUnt6P2BjolInZp028kNHa3cB6Qn6NogFuh3ty5YMBxLKFqqCTtMU517al/4nBYX+W4KtkPdl4aQTvI+kulj4jr2kF0PrEiFbhzMv5yFZXEapChFRwl2kG0VzWMETx7NLvPQM83Q/u+WG4vjAHVAO56BLqyLBigtJ1BL+7nz0oKmr80InYk8/VbcZOsim1arJNstSJBn/8CU7Xikg==");
}

#[test]
fn test_circuit_entities_empty() {
    // No conditions, no wire connections
    // power switch off/on
    roundtrip_blueprint_test("0eNqNkt1uwjAMhd/Fl1NAUMpfxJtMU5W2ZlhqkyhJYRXKu88pW9kGG7uM4/MdH8tnKJsOrSMdQJ6BKqM9yOczeHrVqkm10FsECRSwBQFatellzQndxJ8oVAeIAkjX+AZyHl8EoA4UCC+c4dEXumtLdNwwEpSjcGgxUDWpTFuSVsE4NrDGs9roZM3EfLGdLgX0IFc5+/B8wZmmKPGgjsQC7rqSCv6uB7VPHx7TOxV9UCngTICx6NSFD08QYxQ3M2b3U96OlmfXwS5NBRsFVu5V4/EOejGia6yoZvif2fPZo+wfmB/BvyVmB6vc4CBhB6lgexZ0OhR7Z9qCtO24NbgO7+4j//c+8t/2MbBvycuR/Dnxg1vYjPuYLmO6teEq5ZcjFnBE5wdNtpnn6222Xs0XvMlZjO+UvgAS");
}

#[test]
fn test_circuit_entities() {
    // Test with connections made, values configured, constant combi off
    roundtrip_blueprint_test("0eNrFVNtu2zAM/Rc+DnYRy27iGvuTITAUmVmJ2ZKgS9Ag8L+Xsrc0aNK6yx72YoOWeMhzjskT7PqI1pEO0JyAlNEemh8n8PRTyz59C0eL0AAFHCADLYcUpXtB6pArM+xIy2AcjBmQ7vAFmmLMFhGko/A8YCB1G0OM2wxQBwqEc0tTcGx1HHbouMgCVAbWeM42OvXAiFX59PCYwRGa9ZrrMIXgTN/u8FkeiBP41htSy8fdlO3TwZ6cD+0CJW+lwtwrQs1vDn4lPh4T1FKudaaLKtW7AjAWnZx5wDfOMDHYGO4GHGfuGtWZXZEeDrtLmambJFbkVKQwhSLZenFcjVvGEl9LLt4li/fH29TZlcviTKhDRR26zy2uVksW/4a5x19lLFuRG4cwg08jAM0qBYOVbmqqge9fNik6qSkOuSjrGdIeubGoQ7t3ZmhJMwY0e9l7/BfbbktbfjrNN8Znc9b24fEDdffUB3QfrJADuRD5y9vATDfyzoSZfUxq5qKoNlVdrqv6YqHwMiDfpmb+qHHFp/pbPvX/5HO3nfOkJD3Sr9RcLPAMDtzrxFDUXPRJbNZFyTOxGsdXAG0DIA==");
}

#[test]
fn test_power_switch() {
    // Test power switch with copper wire only
    roundtrip_blueprint_test("0eNqVkc1uwjAMgN/F5xT1J5QtVx4DTSgUj1lK0ypJ11VV3n1OQQhGL7vFsf35szzDyQzYO7IB1AzUdNaDOszg6WK1SX9h6hEUUMAWBFjdpqjvRnSZHyk0XxAFkD3jD6gifghAGygQXjlLMB3t0J7QccE6QXDouamzaSKDpKwETKDqmuEsZbFJWZ/S+yF/RNMZVClgJIfLO08O+6H4W1M91zD3Ovzogw5s9KmNxyhelMu7sm+1MRkadnHUZH1ncM1cbrY39802rgCr/wLrJyBvt9xCPZxOwDc6v/SUb4XcvZe7uqikzPMYfwHfIZ9x");
    // Test power switches with copper/green wire connection
    roundtrip_blueprint_test("0eNqVkt1uwyAMhd/F16TKD003bvsYU1WR1OuQCERAlkUR7z6TTlXa5aY3CGPz+RzLMzR6wN4pE0DMoFprPIiPGby6GqnTW5h6BAEqYAcMjOxS1NsRXeZHFdoviAyUueAPiCKeGKAJKii8cZZgOpuha9BRwTaBUejpkzWpI4E4rxhMIOqa4CTKYJuyPqWLdFwdoll3UBcQZTxR+XHInxMEG5XD5Z4njceheK7hjzUEuok7+yADKf6U2mNk/yyVd0tpeEGakLW2a5SRwbotZ8Vu/+dtt3/FHQ03xg0B1V2A76TWGWrCOdVmvdW4JYCvBWwA+avA+gFI412WRax2i8E3Or/8Kd8KfngvD3VRcZ7nMf4CBabVSw==");
}

#[test]
fn test_control_behavior() {
    // Inserter connected to logistic network
    roundtrip_blueprint_test("0eNqFkVFOwzAMhu/i53RaR7tBrjKhqE29YS11qsQdlKoH4BacjZOQdNLEC9qjnd///9mZoXUjDoFYQM9A1nMEfZwh0pkbl3syDQgaSLAHBdz0uYrS2EtBHDEIBlgUEHf4AbpcXhUgCwnhzWktJsNj3yalLv/zUDD4mMY859RkVdX7Ta1gAr2vN3WKSHASvDMtvjVX8iHrnD9TFLImPXb36ROFKObRDt5RV5xGdJk/YnZ4NIMObWJgsoWlYEcSWMH6oQmNZCT4+fq+9ZiT1og3d0ZGeffhAlrCiEnjrxgCdWjWU6Tsz5RR5QuuqfrP5yhI2riut3suq8PL7rAvn6pqu12WX19EnX8=");
    // Inserter connected to circuit net
    roundtrip_blueprint_test("0eNqdU21OwzAMvYt/t2gd24BeBaEqTc1mkTqVkyLG1ANwC35wMk6CExgfYjDEj0aK/Z798uzuoHUjDkIcod4BWc8B6ssdBFqzcSkWtwNCDRSxhwLY9OmWcNFwLK3vW2ITvcBUAHGHd1BX01UByJEi4Wu5fNk2PPYtigJ+LVTA4INyPaf+Wm+xPD9ZFrCFerU8WWofZTHahAgJUqVDsPvcivQ2TzrWgsgHMtM0Fd+Ezd+FqSp7UxIHlIgHNV180xTFu6bFjbklfYbiLIkdKTaa697J1yQhNkcMxn6I27I1IuiSswFTjWOsUQzT2JfXI7rSostUdXYwkp2t4fnhKcWcX1OIZP+hLHhHXe7wd13odFrimWz5ZskBYY859uaYoOmajdHSyVedkg46yogfiJzMsN53mHdqnwqoX5qf6rrHPfE1sh8T8TAefavdYE/WuDJYQrZYDloCpr9vYPXjBlZpAwvwtyhCHX7Rm1lZSf3p/yxAsSGPan5eLc4u5mer6nSxmM2m6QWy0lIq");
    // Inserter circuit net mode of operation none
    roundtrip_blueprint_test("0eNp1j8FuwzAMQ/+FZ69o2qRd/SvDYDiptglL5MBWigWB/312etllRwrUI7mhHxeaI4vCbuAhSIJ925D4U/xYb7rOBAtWmmAgfqoqqR++X1gSRaWIbMBypx/YJr8bkCgr05O0i9XJMvXFaZv/GAZzSOUtSE0tqLa7HTqDFfbSHboSUcppDKPr6cs/OMTqGzgOC6ubwp1c+HBhpuifkHOuXfbe9s9MgwfFtDtOr017vZ2ul+bctsdjzr92QVuk");
    let bp = BlueprintCodec::decode_string("0eNp1j8FuwzAMQ/+FZ69o2qRd/SvDYDiptglL5MBWigWB/312etllRwrUI7mhHxeaI4vCbuAhSIJ925D4U/xYb7rOBAtWmmAgfqoqqR++X1gSRaWIbMBypx/YJr8bkCgr05O0i9XJMvXFaZv/GAZzSOUtSE0tqLa7HTqDFfbSHboSUcppDKPr6cs/OMTqGzgOC6ubwp1c+HBhpuifkHOuXfbe9s9MgwfFtDtOr017vZ2ul+bctsdjzr92QVuk").unwrap();
    if let factorio_blueprint::Container::Blueprint(mut bp) = bp {
        let ins = bp.entities.get_mut(0).unwrap();
        let mut cb = ins.control_behavior.clone().unwrap();
        cb.circuit_mode_of_operation =
            Some(factorio_blueprint::objects::CircuitModeOfOperation::TWO);
        ins.control_behavior = Some(cb);
        let s =
            BlueprintCodec::encode_string(&factorio_blueprint::Container::Blueprint(bp)).unwrap();
    }
    // Train stop, all circuit checkboxes checked
    roundtrip_blueprint_test("0eNqNUsFuwyAM/RefqZS0UbJx3XXH3qYJkcTbLCUQgYkWRfn3AamqtduhJ+Tn954fhhXaIeDkyDDIFaizxoN8W8HTp9FDwniZECQQ4wgCjB5TxU6TOXi2E2wCyPT4DbLc3gWgYWLC3SUXizJhbNFFwn96AZP1UWJNmhZtqjryFpDNKVr35LDbm7WAGI+dHVSLX3om65KiI9cFYoVGtwOqnnw6QbILKMCh7tWHs6PKI2/gNH7C/raTi2vrzxpmchwicr3Jzjic0x488m7m1UAj8Y3nBXvc8jVZ5qAXfWeDuffM2OOeL7ClnKz3lUIkjNpE5l3uIj1lfnL564cImNH5rDw+lVXzfGzq8lRVRbFtP6bNzHs=");

    // Pump with circuit and logistic condition
    roundtrip_blueprint_test("0eNq1ktFuwyAMRf/Fz3RquizZ0P5kmhAhbmo1QAROuyji3wepVG1PlSbt0fa9l2NghW6ccQrkGOQKZLyLID9WiDQ4PZYeLxOCBGK0IMBpW6ppthMkAeR6/AJZpU8B6JiY8ObfikW52XYYsuC3U8DkYxZ7V07IAXXTCFhAtvunlxzbU0BzG+d+huLgR9XhSV/Ih+IxFMxMrPKsvwcdKURWD9BZu3NBj1i8j9Q6EJ8sMpmd8bYjpzkDpAJlJx22SsJ76Yx+oJiFf2A6jjP1u6sesuc/0HKcyxeq2Ks7pUO++nAGyWHGVB5wi5c/foSAC4a4LXJ4rer27dA21XNd7/cpfQMPLsQ0");

    // Roboport all checked
    roundtrip_blueprint_test("0eNqlkdFOwzAMRf/Fz0FaS2lHfoIPmFCUdtawlMVV4lRUVf6dpJMYgifg0db18b32BqNLOAfyAnoDmthH0KcNIl28dbUn64yggQSvoMDba60CjzxzEMgKyJ/xHXSTXxWgFxLCG2MvVuPTdcRQBD+nFcwcywD7uqlAuv6oYAXdDwVczEhgZ0Z8swtxqJqA9mzqvJgoVsoiCQkV2MWSs6ND4/hCUWgynGRORfY9yUJBUul82rkpHl5qGGGx7u+Mp8q4e6nnLP6mmvDXrPbu51+cBnKuv9k/qL88XMGCIe7Hb49NNzy3Q988dt3hkPMHCCO8oA==");
    // Roboport but no total_logistic_output_signal
    roundtrip_blueprint_test("0eNptUMtqxDAM/Jc5+7BJw6b1r5RinF3RGhLLyPKyIfjfa6eXQnvTiHlJB5a1UJIQFfZAuHHMsO8HcviMfu073RPBIihtMIh+60h44cSiqAYh3ukJO9QPA4oaNNCPxwl2F8u2kDTCX7VB4twEHHtSM5nm0WCHvc7NuJVR4dUt9OUfgaVzhPzddb26rF5bkEohA2X1q+v1G751R8dFU2m0fy+ptdc9R/vrBwYPknz2GV+HaX4b5+vwMk2XS63fUapnvA==");
}

#[test]
fn test_belt_control_behavior() {
    // One belt in pulse, one in hold, and one enable/disable
    roundtrip_blueprint_test("0eNrNVO1qwzAMfBf9dkrTJs1m9iajBCdWW7PENrLTtRS/++yEleyLssJgf2IkS6fzneMLNN2AlpT2wC+gWqMd8OcLOLXXoks5f7YIHJTHHhho0acIT5bQucyT0M4a8lmDnYfAQGmJJ+B52DJA7ZVXOCGOwbnWQ98gxYJbWAyscbHd6MQiQhZluSgZnIFvqkUZR0lF2E4FKwaRuifT1Q0exFEZSl2tonZQvkYtmg5rqVxage9E55BdtwmFrA9CyzqBRKKRsadhVvGen0p7IyPIMoxD9cTBpXl5+uwJUc+PrCTwddiGENgXGVZ3yFDdK0Pck1eonSLn6xs+vwqPlDWCCLvkrsOEcavLkpHDSClzrULdYmZF+wKjYL0VJHwiBk9j5gePPhrwnUWfXPwzj9Z3eLT5V1c1/40Mq5kEKc5HWeLvPJrMZy8GgyOSm071kBfV46ra5OuiWC5DeAMFooiD");
}
