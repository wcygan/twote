# Docker Setup

Firstly, I have no idea how to correctly use Docker. This is the first time I'm trying it out. By separating these docker images out, it seems that I'm able to get a quicker feedback loop.

## Initial Setup

```bash 
chmod +x ./scripts/bootstrap_docker.sh && ./scripts/bootstrap_docker.sh 
```

See [bootstrap_docker.ps1](scripts/windows/bootstrap_docker.ps1) for windows.

## Running the application

Build & run the application:
```bash
docker-compose up -d
```

## Restarting a service

Let's say I make code changes to `accounts-backend`. I can restart the service (while the other services are still running) by running:

```bash
docker-compose down accounts-backend \
  && docker-compose build accounts-backend \
  && docker-compose up -d accounts-backend
```

If I want to do this for my entire application, I can run the same commands without the service parameter:

```bash 
docker-compose down \
  && docker-compose build \
  && docker-compose up -d
```

I personally have this aliased to `dcr` (docker-restart) in my `.bashrc` file.

This command can take arguments, so I can run `dcr accounts-backend` to restart just that service. Or I can run it without arguments to restart the entire application!

```bash
dcr() {
  local service=$1
  docker-compose down $service \
    && docker-compose build $service \
    && docker-compose up -d $service                                              
}   
```

You can do the same in windows with this powershell function:

```bash
function dcr ([string]$service) {
    docker-compose down $service
    if ($LASTEXITCODE -ne 0) { return }
    
    docker-compose build $service
    if ($LASTEXITCODE -ne 0) { return }
    
    docker-compose up -d $service
}
```

## Adding dependencies

If you're adding a dependency to a service, you'll need to rebuild the base images. You can do this by running:

```
./scripts/bootstrap_docker.sh
```