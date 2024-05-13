<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <title>Projet 2 SR06</title>
    <link rel="stylesheet"
          href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css">
</head>

<?php

    include('config.php');

    if ($_SERVER["REQUEST_METHOD"] == "POST") {
    $nom = $_POST["nom"];

    $conn = new mysqli($servername, $username, $password, $dbname);
    if ($conn->connect_error) {
        die("Échec de la connexion à la base de données: " . $conn->connect_error);
    }

    // Requête préparée pour éviter les injections SQL
    $sql = "SELECT * FROM web WHERE nom LIKE ?";
    $stmt = $conn->prepare($sql);

    // Vérification de la préparation de la requête
    if ($stmt === false) {
        die("Erreur de préparation de la requête: " . $conn->error);
    }

    // Liaison du paramètre et exécution de la requête
    $param = "%$nom%";
    $stmt->bind_param("s", $param);
    $stmt->execute();

    // Récupération des résultats
    $result = $stmt->get_result();

    if ($result->num_rows > 0) {
        while ($row = $result->fetch_assoc()) {
            echo "<div class='alert alert-success' role='alert'>";
            echo "Nom: " . $row["nom"];
            echo " - Semestre: " . $row["Semestre"];
            echo "</div>";
        }
    } else {
        echo "<div class='alert alert-danger' role='alert'>";
        echo "Aucun résultat trouvé pour ce nom.";
        echo "</div>";
    }

    $conn->close();
}
?>


<body>
    <div class="container">
        <h1 class="mt-5">Recherche de nom</h1>
        <form action="v1" method="post">
            <div class="form-group">
                <label for="nom">Tapez un nom à rechercher dans la base de données :</label>
                <input type="text" id="nom" name="nom" class="form-control" required>
            </div>
            <button type="submit" class="btn btn-primary">Envoyer</button>
        </form>
    </div>
</body>
</html>