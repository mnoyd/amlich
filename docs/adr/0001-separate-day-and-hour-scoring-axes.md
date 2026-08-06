# Separate Day and Hour Scoring Axes

Amlich scoring uses shared policy mechanics for explainability and consistency, including feature observations, availability handling, weighted aggregation, contributions, and trace output. Day assessment and hour ranking keep separate domain axes and feature identifiers: the day policy decides whether a date is suitable, while the hour policy only orders the twelve hour slots within an already-assessed day. This prevents a good hour from changing the day verdict and prevents day-level suitability language from leaking into hour ranking.

## Considered Options

- Fully shared day/hour axes: rejected because day suitability and hour ordering answer different domain questions.
- Fully separate scoring systems: rejected because it would duplicate availability, contribution, and trace mechanics.
