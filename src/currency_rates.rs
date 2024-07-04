use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::collections::HashMap;

pub fn extract_rates_from_xml(file_content: String) -> Result<HashMap::<String, f64>, Box<dyn std::error::Error>> {
    // Create a new XML reader
    let mut reader = Reader::from_str(file_content.as_str());

    let mut buf = Vec::new();
    let mut results = HashMap::<String, f64>::new();

    // Iterate through XML events
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                let mut currency_code = String::new();
                let mut currency_rate = 0_f64;

                match e.name().into_inner() {
                    b"Cube" => {
                        // Handle attributes
                        for attribute in e.attributes() {
                            let attribute = attribute?;
                            let key = String::from_utf8_lossy(&attribute.key.into_inner());
                            let value = String::from_utf8_lossy(&attribute.value);
                            
                            match key.as_bytes() {
                                b"currency" => currency_code = value.into(),
                                b"rate" => {
                                    currency_rate = match value.parse::<f64>() {
                                        Ok(rate) => rate,
                                        Err(e) => {
                                            println!("Error parsing rate: {:?}", e);
                                            0_f64
                                        }
                                    }
                                },
                                _ => (),
                            }

                        }

                        results.insert(currency_code, currency_rate);
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    Ok(results)
}


mod tests {
    use std::fs::{self};

    use crate::currency_rates::extract_rates_from_xml;

    #[test]
    fn it_reads_exchange_rates_from_xml() {
        // Open the XML file
        let file = fs::read_to_string("./data/example-currency-rates.xml").unwrap();

        let rates = extract_rates_from_xml(file).unwrap();

        // values from ./data/example-currency-rates.xml
        assert_eq!(*rates.get("USD").unwrap(), 1.080);
        assert_eq!(*rates.get("JPY").unwrap(), 173.84);
        assert_eq!(*rates.get("BGN").unwrap(), 1.9558);
        assert_eq!(*rates.get("CZK").unwrap(), 25.143);
        assert_eq!(*rates.get("DKK").unwrap(), 7.4588);
        assert_eq!(*rates.get("GBP").unwrap(), 0.84663);
        assert_eq!(*rates.get("HUF").unwrap(), 393.50);
        assert_eq!(*rates.get("PLN").unwrap(), 4.2918);
        assert_eq!(*rates.get("RON").unwrap(), 4.9769);
        assert_eq!(*rates.get("SEK").unwrap(), 11.3375);
        assert_eq!(*rates.get("CHF").unwrap(), 0.9717);
        assert_eq!(*rates.get("ISK").unwrap(), 149.10);
        assert_eq!(*rates.get("NOK").unwrap(), 11.4015);
        assert_eq!(*rates.get("TRY").unwrap(), 35.1736);
        assert_eq!(*rates.get("AUD").unwrap(), 1.6046);
        assert_eq!(*rates.get("BRL").unwrap(), 5.9345);
        assert_eq!(*rates.get("CAD").unwrap(), 1.4705);
        assert_eq!(*rates.get("CNY").unwrap(), 7.8515);
        assert_eq!(*rates.get("HKD").unwrap(), 8.4340);
        assert_eq!(*rates.get("IDR").unwrap(), 17632.08);
        assert_eq!(*rates.get("ILS").unwrap(), 4.0476);
        assert_eq!(*rates.get("INR").unwrap(), 90.1990);
        assert_eq!(*rates.get("KRW").unwrap(), 1491.06);
        assert_eq!(*rates.get("MXN").unwrap(), 19.5801);
        assert_eq!(*rates.get("MYR").unwrap(), 5.0863);
        assert_eq!(*rates.get("NZD").unwrap(), 1.7641);
        assert_eq!(*rates.get("PHP").unwrap(), 63.211);
        assert_eq!(*rates.get("SGD").unwrap(), 1.4594);
        assert_eq!(*rates.get("THB").unwrap(), 39.560);
        assert_eq!(*rates.get("ZAR").unwrap(), 19.7867);
    }
}