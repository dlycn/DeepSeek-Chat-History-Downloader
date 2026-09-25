use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE,
    CACHE_CONTROL, CONTENT_TYPE, COOKIE, ORIGIN, PRAGMA, REFERER, USER_AGENT};

pub fn build_headers(cookie: &str, xsrf: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br, zstd"),
    );
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9,en-GB;q=0.8,en-US;q=0.7,en;q=0.6"),
    );
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        COOKIE,
        HeaderValue::from_str(cookie).expect("invalid cookie header value"),
    );
    headers.insert(ORIGIN, HeaderValue::from_static("https://www.zhihu.com"));
    headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));
    headers.insert(REFERER, HeaderValue::from_static("https://www.zhihu.com/"));
    headers.insert(
        "sec-ch-ua",
        HeaderValue::from_static(
            "\"Chromium\";v=\"154\", \"Microsoft Edge\";v=\"154\", \"Not A(Brand\";v=\"99\"",
        ),
    );
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert(
        "sec-ch-ua-platform",
        HeaderValue::from_static("\"Windows\""),
    );
    headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36",
        ),
    );
    headers.insert("x-requested-with", HeaderValue::from_static("fetch"));
    headers.insert(
        "x-xsrftoken",
        HeaderValue::from_str(xsrf).expect("invalid xsrf header value"),
    );
    headers.insert("x-zse-93", HeaderValue::from_static("101_3_3.0"));
    headers.insert(
        "x-zse-96",
        HeaderValue::from_static(
            "2.0_bd2PBk+dz2elz2PSCVFrhkolkAZGKrUdb8zk/ih6ozfMdy1mdRsUh15DkD+wbOZQ",
        ),
    );
    headers.insert(
        "x-zst-81",
        HeaderValue::from_static(
            "3_2.0aR_sn77yn6O92wOB8hPZnQr0EMYxc4f18wNBUgpTQ6nxERFZKLY0-4Lm-h3_tufIwJS8gcxTgJS_AuPZNcXCTwxI78YxEM20s4PGDwN8gGcYAupMWufIeQuK7AFpS6O1vukyQ_R0rRnsyukMGvxBEqeCiRnxEL2ZZrxmDucmqhPXnXFMTAoTF6RhRuLPF0YqtrHL6Be9o_x86XO8_9Y_6H30Tqg86XVB8BXLVh3MUbxfpCgBK7NYCqcTvHg88Jxfv9LmpJg1kBOsUrXMbgO9gDVOCULOQTpGZCOfihVOSXcpZqt9ibX11wSGhGL16HwqEGwyprL1r8wsqcxBRwC_beU8icP9YgCYK9V_SDr_fvxmCqfzFhL9J6p0ugtO6qOO-we9fvuK8cP_6exCRbxM2JrGDg2BEBCLD9HfnbLZLDN9ICNBiqgCECp08bL1WBCZ4ucGgcXCguYpVhNfd9FY1vOGthS8whXBBBgC",
        ),
    );

    headers
}