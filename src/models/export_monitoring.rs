#[derive(Clone,Debug,serde::Deserialize)]
pub struct ExportRecords {
    #[serde(rename = "BU / Operation")]
    pub klant: String,
    #[serde(rename = "Type of declaration")]
    pub Type_of_declaration: String,
    #[serde(rename = "Declaration number")]
    pub declaration_number: String, 
    #[serde(rename = "MRN Number")]
    pub mrn: String, 
    #[serde(rename = "Declaration date")]
    pub datum: String, 
    #[serde(rename = "HTS code")]
    pub hts_code: String, 
    #[serde(rename = "Quantity / Collia")]
    pub quantity: String, 
    #[serde(rename = "Gross weight")]
    pub gross_weight: String, 
    #[serde(rename = "Netweight")]
    pub net_weight: String, 
    #[serde(rename = "Unit value")]
    pub unit_value: String, 
    #[serde(rename = "Unit value valuta")]
    pub unit_value_valuta: String, 
    #[serde(rename = "Total line value")]
    pub total_line_value: String, 
    #[serde(rename = "Total line value valuta")]
    pub total_line_value_valuta: String, 
    #[serde(rename = "Declared customs value")]
    pub declared_customs_value: String, 
    #[serde(rename = "Exchange rate")]
    pub exchange_rate: String, 
    #[serde(rename = "Controlled / dual use goods")]
    pub controllled_dual_goods_value: String, 
    #[serde(rename = "ECCN classification")]
    pub eccn_classification: String, 
    #[serde(rename = "Product number / partnumber")]
    pub product_part_number: String, 
    #[serde(rename = "Part description")]
    pub part_description: String, 
    #[serde(rename = "Country of destination")]
    pub country_dest: String, 
    #[serde(rename = "Country of origin")]
    pub country_origin: String, 
    #[serde(rename = "Incoterm")]
    pub incoterm: String, 
    #[serde(rename = "Incoterm place")]
    pub incoterm_place: String, 
    #[serde(rename = "Exporter of record VAT number")]
    pub exporter_VAT_number: String, 
    #[serde(rename = "Ship to name, address details")]
    pub ship_address: String, 
    #[serde(rename = "Ship to country")]
    pub ship_to_country: String, 
    #[serde(rename = "Outbound PID number (FHC:FDA Number)")]
    pub outbound_pid_number: String,  
     #[serde(rename = "PO number")]
    pub po_number: String, 
    #[serde(rename = "EX closed Y/N")]
    pub EX_Closed_yes_no: String,  
     #[serde(rename = "EX closure date")]
    pub ex_closure_date: String, 
    #[serde(rename = "Customs inspection Y/N")]
    pub customs_inspection: String, 
    #[serde(rename = "Marks & Numbers")]
    pub marks_numbers: String, 
    #[serde(rename = "Declaration status")]
    pub declaration_status: String, 
    #[serde(rename = "Invoicenumber")]
    pub invoice_number: String, 
    #[serde(rename = "Filenumber")]
    pub filenumber: String, 
    #[serde(rename = "Last status change")]
    pub last_status_change: String, 
    #[serde(rename = "Column1")]
    pub column1: String, 
}