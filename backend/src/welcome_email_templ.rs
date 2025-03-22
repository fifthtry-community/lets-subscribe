// TODO: remove these hardcoded content
// this should be configurable
pub(crate) const HTML_BODY: &str = r#"<!DOCTYPE html>
<html>
<head>
    <style>
        body {
            font-family: Arial, sans-serif;
            background-color: #f4f4f4;
            margin: 0;
            padding: 0;
        }
        .email-container {
            background-color: #ffffff;
            margin: 20px auto;
            padding: 20px;
            border: 1px solid #cccccc;
            max-width: 600px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
        }
        .email-header {
            background-color: #f4f4f4;
            color: #ffffff;
            padding: 10px;
            text-align: center;
        }
        .email-body {
            padding: 20px;
            color: #333333;
        }
        .email-footer {
            background-color: #f4f4f4;
            color: #777777;
            text-align: center;
            padding: 10px;
            font-size: 12px;
        }
        .button {
            text-decoration: none;
        }
        .list-item {
            margin-bottom: 20px;
        }
        .list-item-title {
            font-weight: bold;
            margin-bottom: 5px;
        }
        .list-item-body {
            margin-bottom: 10px;
        }
    </style>
</head>
<body>
    <div class="email-container">
        <table width="100%" cellpadding="0" cellspacing="0">
            <tr>
                <td class="email-header">
                    <img class="logo" src="https://ui.fifthtry.com/-/ui.fifthtry.com/assets/fifthtry.png" alt="Company Logo">
                </td>
            </tr>
            <tr>
                <td class="email-body">
                    <h1>Thank you for subscribing + Start Learning</h1>
            <p>Hi {name}, Thank you for subscribing to the FifthTry newsletter! We're excited to have you on board.</p>
            <p>To get you started, here are some recent highlights and resources you might find useful:</p>
                    <ul>
                        <li class="list-item">
                            <div class="list-item-title">Build and Host Your Website on FifthTry</div>
                            <div class="list-item-body">Create your FifthTry account and start building your website right in your browser.</div>
                            <a href="https://www.fifthtry.com/" class="button">Start here</a>
                        </li>
                        <li class="list-item">
                            <div class="list-item-title">The FifthTry Editor</div>
                            <div class="list-item-body">Our intuitive and easy-to-use online editor for all your fastn projects.</div>
                            <a href="https://www.fifthtry.com/" class="button">Check out</a>
                        </li>
                        <li class="list-item">
                            <div class="list-item-title">Tutorial</div>
                            <div class="list-item-body">Follow this step-by-step guide to build your website using our design system package.</div>
                            <a href="https://www.fifthtry.com/" class="button">Read the tutorial</a>
                        </li>
                    </ul>
                    <p>We’re thrilled to help you on your web development journey. If you have any questions or need assistance, reply to this email.</p>
                    <p>Happy coding!</p>
                    <p>Best regards,</p>
                    <p>The FifthTry Team</p>
                </td>
            </tr>
            <tr>
                <td class="email-footer">
                    <p>&copy; 2024 FifthTry. All rights reserved.</p>
                </td>
            </tr>
        </table>
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
