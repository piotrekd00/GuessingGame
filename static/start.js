function startGame() {
    const lowerBound = document.getElementById('lower-bound').value;
    const upperBound = document.getElementById('upper-bound').value;

    fetch('/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ lower_bound: parseInt(lowerBound), upper_bound: parseInt(upperBound) })
    })
    .then(response => response.json())
    .then(gameId => {
        window.location.href = `/game/${gameId}`;
    })
    .catch(error => console.error('Error:', error));
}
