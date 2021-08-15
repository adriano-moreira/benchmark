FROM mcr.microsoft.com/dotnet/sdk AS build

WORKDIR /app
COPY . .
RUN dotnet build --configuration Release


FROM mcr.microsoft.com/dotnet/runtime
WORKDIR /app
COPY --from=build /app/bin /app/bin

ENTRYPOINT ["dotnet","bin/Release/net5.0/CSharpProject.dll"]
