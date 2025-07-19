pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     // fn summarize(&self) -> String {
//     //     format!("{}, by {} ({})", self.headline, self.author, self.location)
//     // }
// }

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}


https://geekskai.com/blog/js/how-to-check-if-character-is-double-quote-in-js/
https://geekskai.com/blog/windows/remote-desktop-windows-11-home/
https://geekskai.com/blog/business/eco-friendly-business-practices-small-companies/
https://geekskai.com/blog/guide/elevate-your-game-basketball-quotes-collection/
https://geekskai.com/blog/guide/george-washington-quotes-wisdom-uncovered/
https://geekskai.com/blog/guide/how-to-pick-the-best-5starsstocks-com-dividend-stocks/
https://geekskai.com/blog/quotes/iamrestaurant-com-quotes/
https://geekskai.com/blog/guide/elevate-your-weekend-friday-inspirational-quotes/
https://geekskai.com/blog/js/how-to-capitalize-first-letter/
https://geekskai.com/blog/js/how-to-convert-csv-to-dynamodb-json-in-javascript/
https://geekskai.com/blog/js/javascript-sleep-function-implement/
https://geekskai.com/blog/guide/unlocking-the-secrets-of-the-vital-magnet-blog/
https://geekskai.com/blog/guide/save-money-with-money6xcom-6x-your-savings/

https://geekskai.com/blog/ai/ai-multi-agent-control-platform/
https://geekskai.com/blog/how-does-color-accuracy-affect-your-experience-with-a-portable-monitor/
https://geekskai.com/blog/how-ai-websites-and-tools-empower-small-businesses/
https://geekskai.com/blog/guide/exclusive-monopoly-go-dice-links-for-ultimate-gaming/
https://geekskai.com/blog/guide/lessons-learned-the-traveler-hired-the-wrong-tour-guide/
https://geekskai.com/blog/guide/how-consumer-goods-intelligence-tools-help-optimize-product-performance-in-the-usa-market/


https://geekskai.com/blog/guide/unlocking-uniuni-tracking-solutions-for-you/
https://geekskai.com/blog/guide/master-averitt-tracking-for-easy-shipment-monitoring/
https://geekskai.com/blog/guide/laugh-out-loud-funny-friendship-quotes-for-your-best-friend/
https://geekskai.com/blog/js/best-ways-check-key-exists-in-javascript-objects/
https://geekskai.com/blog/js/cms-system-next-js/
https://geekskai.com/blog/js/how-to-check-infinity-in-javascript/
https://geekskai.com/blog/js/simplifying-react-query-with-expo-sqlite-integration/
https://geekskai.com/blog/guide/boost-your-midweek-mood-with-50-wednesday-quotes/
https://geekskai.com/blog/js/js-express-tracking-ultimate-guide-for-package-monitoring/
https://geekskai.com/blog/react/ngx-graph-react-guide
https://geekskai.com/blog/guide/god-quotes/
https://geekskai.com/blog/ultimate-crypto-security-features-on-ecrypto1com
https://geekskai.com/blog/guide/unlocking-the-secrets-of-the-vital-magnet-blog
https://geekskai.com/blog/react/master-react-js-swag-shop-setup
https://geekskai.com/blog/js/how-to-convert-csv-to-dynamodb-json-in-javascript
https://geekskai.com/blog/myfavouriteplaces-org-blog



https://geekskai.com/blog/how-to-make-videos-that-drive-sales
https://geekskai.com/blog/css/tips-for-using-css-first-child
