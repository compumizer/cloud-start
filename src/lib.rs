use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn cloud_start(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());

    let body = "
    <html>
    <script src=\"https://cdn.jsdelivr.net/npm/canvas-confetti@1.5.1/dist/confetti.browser.min.js\"></script>
    <script>
        var duration = 15 * 1000;
        var animationEnd = Date.now() + duration;
        var defaults = { startVelocity: 30, spread: 360, ticks: 60, zIndex: 0 };

        function randomInRange(min, max) {
            return Math.random() * (max - min) + min;
        }

        var interval = setInterval(function() {
        var timeLeft = animationEnd - Date.now();

        if (timeLeft <= 0) {
            return clearInterval(interval);
        }

        var particleCount = 50 * (timeLeft / duration);
        confetti(Object.assign({}, defaults, { particleCount, origin: { x: randomInRange(0.1, 0.3), y: Math.random() - 0.2 } }));
        confetti(Object.assign({}, defaults, { particleCount, origin: { x: randomInRange(0.7, 0.9), y: Math.random() - 0.2 } }));
        }, 250); 
    </script>
    <body style=\"background-color: #f7f4f8; height: 100%;\">
        <div style=\"
            left: 0;
            margin-top: -100px;
            position: absolute;
            text-align: center;
            top: 50%;
            width: 100%;
            \">
            <svg width=\"147\" height=\"42\" viewBox=\"0 0 147 42\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">
                <path d=\"M18.1458 3.61366H3.52139V7.42598H13.6396V11.0113H3.52139V18.0967H0V0.0283795H18.1458V3.61366Z\" fill=\"#11203D\"/>
                <path d=\"M40.0799 3.61366H25.4555V7.42598H35.5736V11.0113H25.4555V14.5019H40.0799V18.0967H21.9341V0.0283795H40.0799V3.61366Z\" fill=\"#11203D\"/>
                <path d=\"M61.9954 18.0967H56.5414L50.3348 12.7613H47.3895V18.0967H43.8681V0.0283795H55.7609C56.8821 0.0283795 57.901 0.315328 58.8178 0.889225C59.7407 1.46312 60.4747 2.23252 61.0198 3.19743C61.5711 4.16233 61.8467 5.22814 61.8467 6.39485C61.8467 7.56157 61.5742 8.62738 61.0291 9.59228C60.484 10.5509 59.7562 11.314 58.8456 11.8816C57.9351 12.4492 56.9162 12.7424 55.7888 12.7613L61.9954 18.0967ZM47.3895 3.6042V9.19497H55.7609C56.4671 9.19497 57.071 8.92063 57.5727 8.37196C58.0745 7.82329 58.3253 7.16425 58.3253 6.39485C58.3253 5.62545 58.0745 4.96957 57.5727 4.42721C57.071 3.87854 56.4671 3.6042 55.7609 3.6042H47.3895Z\" fill=\"#11203D\"/>
                <path d=\"M83.9109 0V18.0683H80.3895V6.75433L75.9854 14.5871H73.7091L69.305 6.75433V18.0683H65.7836V0H69.5744L74.8426 9.38416L80.12 0H83.9109Z\" fill=\"#11203D\"/>
                <path d=\"M105.761 0.0283795L98.5884 9.77202V18.0967H95.0671V9.71526L87.6991 0.0283795H92.1589L96.8324 6.16782L101.357 0.0283795H105.761Z\" fill=\"#11203D\"/>
                <path d=\"M116.109 18.0967C117.348 18.0967 118.509 17.8633 119.593 17.3967C120.684 16.9237 121.637 16.2741 122.455 15.4479C123.279 14.6218 123.923 13.6632 124.388 12.5721C124.858 11.4748 125.094 10.3049 125.094 9.06253C125.094 7.81383 124.858 6.64396 124.388 5.55293C123.923 4.46189 123.279 3.5033 122.455 2.67714C121.637 1.85098 120.684 1.20455 119.593 0.737868C118.509 0.264876 117.348 0.0283795 116.109 0.0283795C114.87 0.0283795 113.706 0.264876 112.616 0.737868C111.532 1.20455 110.578 1.85098 109.754 2.67714C108.936 3.5033 108.292 4.46189 107.821 5.55293C107.357 6.64396 107.125 7.81383 107.125 9.06253C107.125 10.3049 107.357 11.4748 107.821 12.5721C108.292 13.6632 108.936 14.6218 109.754 15.4479C110.578 16.2741 111.532 16.9237 112.616 17.3967C113.706 17.8633 114.87 18.0967 116.109 18.0967ZM116.109 3.63258C117.1 3.63258 118.005 3.87854 118.822 4.37045C119.64 4.85605 120.29 5.51194 120.773 6.3381C121.263 7.15795 121.507 8.06609 121.507 9.06253C121.507 10.059 121.263 10.9703 120.773 11.7964C120.29 12.6163 119.64 13.2722 118.822 13.7641C118.005 14.2497 117.1 14.4925 116.109 14.4925C115.118 14.4925 114.214 14.2497 113.396 13.7641C112.578 13.2722 111.925 12.6163 111.436 11.7964C110.953 10.9703 110.711 10.059 110.711 9.06253C110.711 8.06609 110.953 7.15795 111.436 6.3381C111.925 5.51194 112.578 4.85605 113.396 4.37045C114.214 3.87854 115.118 3.63258 116.109 3.63258Z\" fill=\"#11203D\"/>
                <path d=\"M143.479 0.0283795H147V18.0967H142.958L132.394 5.03264V18.0967H128.873V0.0283795H132.914L143.479 13.0924V0.0283795Z\" fill=\"#11203D\"/>
                <path d=\"M48.8706 30.5825C49.5467 30.5825 50.1316 30.6804 50.6252 30.8761C51.1189 31.061 51.5481 31.3165 51.913 31.6427V33.9262C51.6554 33.7305 51.3872 33.5402 51.1081 33.3553C50.8291 33.1705 50.5125 33.0183 50.1584 32.8986C49.815 32.779 49.4072 32.7192 48.935 32.7192C48.2911 32.7192 47.7063 32.8769 47.1804 33.1922C46.6546 33.5076 46.2361 33.9317 45.9248 34.4645C45.6136 34.9973 45.458 35.6062 45.458 36.2913C45.458 36.9654 45.6136 37.5744 45.9248 38.1181C46.2361 38.6509 46.6546 39.0749 47.1804 39.3903C47.7063 39.6948 48.2911 39.847 48.935 39.847C49.654 39.847 50.255 39.7383 50.7379 39.5208C51.2316 39.2924 51.6769 39.0151 52.074 38.6889V40.8909C51.7091 41.228 51.253 41.4998 50.7057 41.7064C50.1584 41.9021 49.5467 42 48.8706 42C48.098 42 47.3682 41.8641 46.6814 41.5922C45.9946 41.3095 45.3883 40.9181 44.8624 40.4179C44.3366 39.9068 43.9234 39.3033 43.623 38.6074C43.3225 37.9115 43.1722 37.1394 43.1722 36.2913C43.1722 35.4431 43.3225 34.6711 43.623 33.9751C43.9234 33.2792 44.3366 32.6812 44.8624 32.181C45.3883 31.6699 45.9946 31.2784 46.6814 31.0066C47.3682 30.7239 48.098 30.5825 48.8706 30.5825Z\" fill=\"#34E8BD\"/>
                <path d=\"M57.7994 30.8272V39.7165H63.192V41.7553H55.5619V30.8272H57.7994Z\" fill=\"#34E8BD\"/>
                <path d=\"M71.4125 30.5825C72.1852 30.5825 72.9095 30.7239 73.5856 31.0066C74.2617 31.2784 74.8573 31.6699 75.3724 32.181C75.8982 32.6812 76.306 33.2792 76.5958 33.9751C76.8963 34.6711 77.0465 35.4431 77.0465 36.2913C77.0465 37.1394 76.8963 37.9115 76.5958 38.6074C76.306 39.3033 75.8982 39.9068 75.3724 40.4179C74.8573 40.9181 74.2617 41.3095 73.5856 41.5922C72.9095 41.8641 72.1852 42 71.4125 42C70.6398 42 69.9101 41.8641 69.2233 41.5922C68.5472 41.3095 67.9462 40.9181 67.4204 40.4179C66.9053 39.9068 66.4975 39.3033 66.197 38.6074C65.9073 37.9115 65.7624 37.1394 65.7624 36.2913C65.7624 35.4431 65.9073 34.6711 66.197 33.9751C66.4975 33.2792 66.9053 32.6812 67.4204 32.181C67.9462 31.6699 68.5472 31.2784 69.2233 31.0066C69.9101 30.7239 70.6398 30.5825 71.4125 30.5825ZM71.4125 32.7192C70.7686 32.7192 70.1891 32.8769 69.674 33.1922C69.1696 33.5076 68.7726 33.9317 68.4828 34.4645C68.1931 34.9973 68.0482 35.6062 68.0482 36.2913C68.0482 36.9654 68.1931 37.5744 68.4828 38.1181C68.7726 38.6509 69.1696 39.0749 69.674 39.3903C70.1891 39.6948 70.7686 39.847 71.4125 39.847C72.0564 39.847 72.6305 39.6948 73.1349 39.3903C73.6393 39.0749 74.0363 38.6509 74.3261 38.1181C74.6158 37.5744 74.7607 36.9654 74.7607 36.2913C74.7607 35.6062 74.6158 34.9973 74.3261 34.4645C74.0363 33.9317 73.6393 33.5076 73.1349 33.1922C72.6305 32.8769 72.0564 32.7192 71.4125 32.7192Z\" fill=\"#34E8BD\"/>
                <path d=\"M89.6937 30.7946V37.433C89.6937 38.379 89.4845 39.1946 89.0659 39.8796C88.6582 40.5538 88.1001 41.0757 87.3918 41.4454C86.6943 41.8043 85.9109 41.9837 85.0417 41.9837C84.1724 41.9837 83.3836 41.8043 82.6754 41.4454C81.9671 41.0757 81.4037 40.5538 80.9852 39.8796C80.5774 39.1946 80.3735 38.379 80.3735 37.433V30.7946H82.611V37.3025C82.611 38.1398 82.831 38.7705 83.271 39.1946C83.7217 39.6078 84.3119 39.8089 85.0417 39.7981C85.7607 39.7981 86.3455 39.5915 86.7962 39.1783C87.247 38.765 87.4723 38.1398 87.4723 37.3025V30.7946H89.6937Z\" fill=\"#34E8BD\"/>
                <path d=\"M98.0913 30.5662C99.2289 30.5662 100.227 30.8 101.085 31.2676C101.944 31.7351 102.609 32.3876 103.081 33.2249C103.564 34.0621 103.806 35.0462 103.806 36.1771C103.806 37.2536 103.58 38.2322 103.13 39.113C102.679 39.9829 102.035 40.6734 101.198 41.1845C100.361 41.6955 99.363 41.9511 98.204 41.9511C97.4635 41.9511 96.7231 41.913 95.9826 41.8369C95.2529 41.7608 94.5607 41.6575 93.9061 41.527V30.9903C94.5714 30.8816 95.2529 30.7837 95.9504 30.6967C96.6479 30.6097 97.3616 30.5662 98.0913 30.5662ZM98.1396 32.6866C97.8177 32.6866 97.4796 32.7029 97.1255 32.7355C96.7714 32.7682 96.4441 32.8117 96.1436 32.866V39.6513C96.4655 39.7165 96.7982 39.7654 97.1416 39.7981C97.4957 39.8198 97.8284 39.8307 98.1396 39.8307C98.8479 39.8307 99.4542 39.6784 99.9586 39.374C100.463 39.0586 100.849 38.6291 101.118 38.0854C101.386 37.5417 101.52 36.9328 101.52 36.2586C101.52 35.5627 101.386 34.9484 101.118 34.4155C100.849 33.8827 100.463 33.4641 99.9586 33.1596C99.4542 32.8443 98.8479 32.6866 98.1396 32.6866Z\" fill=\"#34E8BD\"/>
            </svg>
            <p style=\"color: #848fa8;
                font-size: 16px;
                font-family: Sen,Europa,Avenir,system,-apple-system,'.SFNSText-Regular',San Francisco,Segoe UI,Helvetica Neue,Lucida Grande,sans-serif;
            \">
                Congratulations! Your first Spin Application &mdash; powered by WebAssembly &mdash; is ready to go!
        </div>
        <div style=\"
            position: absolute;
            bottom: 0;
            width: 100%;
            \">
            <p style=\"color: #848fa8;
            font-size: 10px;
            font-family: Sen,Europa,Avenir,system,-apple-system,'.SFNSText-Regular',San Francisco,Segoe UI,Helvetica Neue,Lucida Grande,sans-serif;
            \">
                Confetti provided by https://github.com/catdad/canvas-confetti
            </p>
        </div    
    </body>
    </html>";

    Ok(http::Response::builder()
        .status(200)
        .body(Some(body.into()))?)
}
