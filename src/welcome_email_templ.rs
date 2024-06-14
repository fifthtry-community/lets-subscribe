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
            display: flex;
            flex-direction: column;
            gap: 8px;
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
        	text-decoration: none;
        }

        .content regards{
        	margin-top: -24px;
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
            <h1>Thank you for subscribing + Start Learning</h1>
            <p>Hi {name}, Thank you for subscribing to the FifthTry newsletter! 🎉 We're excited to have you on board.</p>
            <p>To get you started, here are some recent highlights and resources you might find useful:</p>
            <ul>
            	<li style="list-style-type: decimal; font-weight: 600;">Build and Host Your Website on FifthTry</li>
            	<p>Create your FifthTry account and start building your website right in your browser. <a href="fifthtry.com">Start here</a></p>
            	<li style="list-style-type: decimal; font-weight: 600;">The FifthTry Editor</li>
            	<p>Our intuitive and easy-to-use online editor for all your fastn projects. <a href="fifthtry.com">Check out</a></p>
            	<li style="list-style-type: decimal; font-weight: 600;">Tutorial</li>
            	<p>Follow this step-by-step guide to build your website using our design system package.<a href="fastn-community.github.io/design-system/">Read the tutorial</a></p>
            </ul>
            <div class="regards">
            <p>We’re thrilled to help you on your web development journey. If you have any questions or need assistance, reply to this email.</p>
            <p>Happy coding! 🚀</p>
            <p>Best regards,</p>
            <p>Team FifthTry</p>
            </div>
        </div>
            

            
        
        <div class="footer">
            
            <p>© 2024 FifthTry. All rights reserved.</p>
        </div>
    </div>
</body>
</html>
"#;

pub(crate) const TEXT_BODY: &str = r#"Thank you for subscribing + Start Learning
Hi {name}, Thank you for subscribing to the FifthTry newsletter! 🎉 We're excited to have you on board.
To get you started, here are some recent highlights and resources you might find useful:

    - Build and Host Your Website on FifthTry.
    - Create your FifthTry account and start building your website right in your browser. Start at www.fifthtry.com.
    - The FifthTry Editor: Our intuitive and easy-to-use online editor for all your fastn projects.
    - Follow this step-by-step guide to build your website using our design system package. Read the tutorial at: fastn-community.github.io/design-system/

We’re thrilled to help you on your web development journey. If you have any questions or need assistance, reply to this email.
Happy coding! 🚀

Best regards,
Team FifthTry

© 2024 FifthTry. All rights reserved.
"#;
