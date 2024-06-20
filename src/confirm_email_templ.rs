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
            border-radius: 5px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
            overflow: hidden;
        }
        .header, .footer {
            text-align: center;
            padding: 20px;
            background-color: #f4f4f4;
        }
        .header img {
            max-width: 150px;
        }
        .content {
            padding: 20px;
            text-align: center;
        }
        .content h1 {
            font-size: 24px;
            color: #333333;
            margin-bottom: 20px;
        }
        .content p {
            font-size: 16px;
            color: #666666;
            margin-bottom: 20px;
        }
        .content a {
            display: inline-block;
            padding: 10px 20px;
            background-color: #28a745;
            color: #ffffff;
            text-decoration: none;
            border-radius: 5px;
            font-size: 16px;
        }
        .footer p {
            font-size: 12px;
            color: #999999;
        }
        .footer a {
            color: #999999;
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <table class="container" role="presentation" cellpadding="0" cellspacing="0" width="100%">
        <tr>
            <td class="header">
                <img src="https://ui.fifthtry.com/-/ui.fifthtry.com/assets/fifthtry.png" alt="Company Logo">
            </td>
        </tr>
        <tr>
            <td class="content">
                <h1>Confirm Your Subscription</h1>
                <p>Hi {name}, Thank you for subscribing {topic}!</p>
                <p>We're thrilled to have you join our community. You're now just one step away from staying updated with our latest news, tips, and exclusive content delivered straight to your inbox.</p>
                <p>Please confirm your subscription by clicking the link below:</p>
                <a href="{confirmation_link}">Confirm Subscription</a>
            </td>
        </tr>
        <tr>
            <td class="footer">
                <p>We respect your privacy and promise to keep your information safe. If you didn't subscribe to this newsletter or have any concerns, please ignore this email or reach out to us at <a href="https://example.com/unsubscribe">help@fifthtry.com</a>.</p>
                <p>&copy; 2024 FifthTry. All rights reserved.</p>
            </td>
        </tr>
    </table>
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
