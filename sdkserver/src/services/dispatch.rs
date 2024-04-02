use prost::Message;
use proto::{Dispatch, Gateserver, RegionInfo};

pub const QUERY_DISPATCH_ENDPOINT: &str = "/query_dispatch";
pub const QUERY_GATEWAY_ENDPOINT: &str = "/query_gateway";

#[tracing::instrument]
pub async fn query_dispatch() -> String {
    let rsp = Dispatch {
        retcode: 0,
        region_list: vec![RegionInfo {
            name: String::from("RobinSR"),
            title: String::from("RobinSR"),
            env_type: String::from("9"),
            dispatch_url: String::from("http://127.0.0.1:21000/query_gateway"),
            ..Default::default()
        }],
        ..Default::default()
    };

    let mut buff = Vec::new();
    rsp.encode(&mut buff).unwrap();

    rbase64::encode(&buff)
}

#[tracing::instrument]
pub async fn query_gateway() -> String {
    let rsp = Gateserver {
        retcode: 0,
        ip: String::from("127.0.0.1"),
        port: 23301,
        asset_bundle_url: String::from(
            "https://autopatchcn.bhsr.com/asb/BetaLive/output_6785106_15237df2ef89",
        ),
        lua_url: String::from(
            "https://autopatchcn.bhsr.com/lua/BetaLive/output_6785460_26c4b6c61a8b",
        ),
        ex_resource_url: String::from(
            "https://autopatchcn.bhsr.com/design_data/BetaLive/output_6787319_5f3f1dae4769",
        ),
        ifix_version: String::from("0"),
        lua_version: String::from("6785460"),
        jblkncaoiao: true,
        hjdjakjkdbi: true,
        ldknmcpffim: true,
        feehapamfci: true,
        eebfeohfpph: true,
        dfmjjcfhfea: true,
        najikcgjgan: true,
        giddjofkndm: true,
        fbnbbembcgn: false,
        dedgfjhbnok: false,
        use_tcp: true,
        linlaijbboh: false,
        ahmbfbkhmgh: false,
        nmdccehcdcc: false,
        ..Default::default()
    };

    let mut buff = Vec::new();
    rsp.encode(&mut buff).unwrap();

    rbase64::encode(&buff)
}
