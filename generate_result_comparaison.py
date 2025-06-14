import csv
import matplotlib.pyplot as plt

colors = {
    "Hasting Metropolis": "green",
    "Rejet": "red",
    "Tie": "yellow",
}

with open("results_comparaison.csv") as data:
    reader = csv.reader(data)

    X = next(reader)[1:]  # itérations
    color_labels = next(reader)[1:]
    Y = list(map(float, next(reader)[1:]))

    bar_colors = [colors[label] for label in color_labels]

    bars = plt.bar(X, Y, color=bar_colors)

    from matplotlib.patches import Patch
    legend_elements = [
        Patch(facecolor='red', label='Rejet'),
        Patch(facecolor='green', label='Hasting Metropolis'),
        Patch(facecolor='yellow', label='Même résultats')
    ]

    plt.xlabel("Itérations")
    plt.ylabel("Distance moyenne")
    plt.title("Comparaison de la méthode du rejet et d'Hastings Metropolis")
    plt.grid(True, axis='y')
    plt.legend(handles=legend_elements)
    plt.xticks(rotation=90) 
    plt.tight_layout()
    plt.show()
