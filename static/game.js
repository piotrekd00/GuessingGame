let playerName = '';

function submitName() {
    playerName = document.getElementById('player-name').value;
    document.getElementById('name-input').style.display = 'none';
    document.getElementById('guessing-game').style.display = 'block';
    document.getElementById('scores').style.display = 'block';
    fetchScores();
}

function makeGuess() {
    const guess = document.getElementById('guess').value;

    fetch(`/guess/${gameId}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ player_name: playerName, num: parseInt(guess) })
    })
    .then(response => response.text())
    .then(data => {
        document.getElementById('result').textContent = data;
        if (data.startsWith("Congratulations")) {
            document.getElementById('restart-button').style.display = 'block';
        }
        fetchScores();
    })
    .catch(error => console.error('Error:', error));
}

function home() {
    window.location.href = '/';
}

function fetchScores() {
    fetch(`/game/${gameId}/scores`)
    .then(response => response.json())
    .then(data => {
        const topScores = data.sort((a, b) => a.attempts - b.attempts).slice(0, 10);
        const scoresList = document.getElementById('scores-list');
        scoresList.innerHTML = '';
        topScores.forEach(score => {
            const listItem = document.createElement('li');
            listItem.textContent = `${score.name}: ${score.attempts} attempts`;
            scoresList.appendChild(listItem);
        });
    })
    .catch(error => console.error('Error:', error));
}
