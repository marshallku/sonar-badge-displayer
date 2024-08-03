# Sonar Badge Displayer

Sonar Badge Displayer is a Rust-based API that allows you to display SonarQube badges for your projects without exposing your SonarQube token.

## Features

- Fetches and displays SonarQube badges for specified projects and metrics
- Securely handles SonarQube authentication without exposing tokens
- Built with Axum web framework for efficient async handling

## Usage

1. Set up your environment variables (see Configuration section).

2. Run the application:

   ```bash
   docker compose up --build -d
   ```

3. Access badges via the API:

   ```txt
   http://localhost:3000/?project=your_project_key&metric=your_metric
   ```

   Replace `your_project_key` with your SonarQube project key and `your_metric` with the desired metric (e.g., `coverage`, `bugs`, `code_smells`, etc.).

## Configuration

Set the following environment variables:

- `SONARQUBE_URL`: Your SonarQube instance URL
- `SONARQUBE_TOKEN`: Your SonarQube authentication token

## API Endpoints

- `GET /`: Fetches and displays the SonarQube badge
  - Query Parameters:
    - `project`: SonarQube project key (required)
    - `metric`: SonarQube metric to display (required)
