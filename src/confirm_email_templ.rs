// TODO: remove these hardcoded content
// this should be configurable
pub(crate) const HTML_BODY: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Confirm Your Subscription</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            background-color: #f4f4f4;
            margin: 0;
            padding: 0;
        }
        .container {
            max-width: 600px;
            margin: 0 auto;
            background-color: #ffffff;
            padding: 20px;
            border-radius: 5px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
        }
        .header {
            padding: 20px 0;
            border-bottom: 1px solid #dddddd;
            background-color: #F0F0F0;
            display: flex;
            flex-direction: column;
            align-items: center;
        }
        .logo {
            width: 200px;
            margin-left: 6px;
        }
        .content {
            padding: 20px;
            text-align: center;
            display: flex;
            flex-direction: column;
            gap: 8px;
            align-items: center;
        }
        .content h1 {
            font-size: 30px;
            color: #333333;
        }
        .content p {
            font-size: 16px;
            color: #303030;
        }
        .content a {
            display: inline-block;
            margin-top: 20px;
            margin-bottom: 4px;
            padding: 16px 20px;
            background-color: #ef8435;
            color: #ffffff;
            text-decoration: none;
            border-radius: 8px;
            width: 250px;
            text-align: center;
        }

        .concern p {
            font-size: 12px;
            color: #666666;
        }


        .footer {
            text-align: center;
            padding: 20px 0;
            border-top: 1px solid #dddddd;
            font-size: 12px;
            color: #999999;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <img class="logo" src="https://www.fifthtry.com/-/ui.fifthtry.com/assets/logo.svg" alt="Company Logo">
        </div>
        <div class="content">
            <h1>Confirm Your Subscription</h1>
            <p>Hi {name}, Thank you for subscribing to the {topic}!</p>
            <p>We're thrilled to have you join our community. You're now just one step away from
            staying updated with our latest news, tips, and exclusive content delivered straight to
            your inbox.</p>
            <p>Please confirm your subscription by clicking the link below:</p>

            <a href="{confirmation_link}">Confirm Subscription</a>
        </div>
            <div class="concern">
            <p>We respect your privacy and promise to keep your information safe. If you didn't
            subscribe to this newsletter or have any concerns, please ignore this email or reach
            out to us at help@fifthtry.com.</p>
            </div>

        <div class="footer">
            <p>© 2024 FifthTry. All rights reserved.</p>
        </div>
    </div>
</body>
</html>
"#;

pub(crate) const TEXT_BODY: &str = r#"Confirm Your Subscription

Hi {name}, Thank you for subscribing {topic}!
We're thrilled to have you join our community. You're now just one step away from staying updated with our latest news, tips, and exclusive content delivered straight to your inbox.

Please confirm your subscription by clicking the link below:

{confirmation_link}


We respect your privacy and promise to keep your information safe. If you didn't subscribe to this newsletter or have any concerns, please ignore this email or reach out to us at help@fifthtry.com.


© 2024 FifthTry. All rights reserved.
"#;
