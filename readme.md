# Status Change Monitor

A CLI tool which queries the websites to see if some element has changed. If it has, it informs recipient on email (using mailgun).

The idea came after I saw a product in a web store, that I really want to buy, being sold out and instead of checking every single day when the status of a product change, I have decided to automate the process. 

In a production this CLI tool is fired via cron.

Also this was my first cool project written in Rust.

## How to use

CLI tool takes two arguments - path to a JSON file and path to a log file. 

JSON file is a configuration object with data about sites and mailgun

An example of it (notice `sites` is array of object, meaning you can define multiple sites for monitoring status change):

```json
{
    "sites": [
        {
            "description": "The Everything Bundle from Bake With Jack",
            "url": "https://www.bakewithjack.co.uk/shop/the-everything-bundle",
            "selector": "#productDetails .product-mark.sold-out",
            "happy_note": "The product is in stock! :) Hurry and visit <a href=\"https://www.bakewithjack.co.uk/shop/the-everything-bundle\">https://www.bakewithjack.co.uk/shop/the-everything-bundle</a>",
            "disappointing_note": "The product is still out of stock. :("
        }
    ],
    "mailgun": {
        "from": "Status Change Monitor <status.change.monitor@jernejsila.com>",
        "to": "jernej.sila@gmail.com",
        "domain": "<actual-domain>",
        "api_key": "<actual-key>"
    }
}
```

Currently, status change is defined only by the absence of element defined by CSS selector.