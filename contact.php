<?php
if ($_POST) {
    $name = htmlspecialchars($_POST['name']);
    $email = htmlspecialchars($_POST['email']);
    $message = htmlspecialchars($_POST['message']);
    
    echo "<h2>Thank you, $name!</h2>";
    echo "<p>We received your message and will contact you at $email soon.</p>";
} else {
?>
<!DOCTYPE html>
<html>
<head>
    <title>Contact - DevForge</title>
    <style>
        body { font-family: Arial, sans-serif; max-width: 600px; margin: 50px auto; padding: 20px; }
        input, textarea { width: 100%; padding: 10px; margin: 10px 0; border: 1px solid #ddd; border-radius: 5px; }
        button { background: #2c3e50; color: white; padding: 12px 30px; border: none; border-radius: 5px; cursor: pointer; }
        button:hover { background: #34495e; }
    </style>
</head>
<body>
    <h1>Contact DevForge</h1>
    <form method="POST">
        <input type="text" name="name" placeholder="Your Name" required>
        <input type="email" name="email" placeholder="Your Email" required>
        <textarea name="message" placeholder="Your Message" rows="5" required></textarea>
        <button type="submit">Send Message</button>
    </form>
</body>
</html>
<?php } ?>
