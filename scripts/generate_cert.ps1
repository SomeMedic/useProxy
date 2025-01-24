# Генерация самоподписанного сертификата для UseProxy
$cert = New-SelfSignedCertificate `
    -Subject "CN=localhost" `
    -DnsName "localhost" `
    -KeyAlgorithm RSA `
    -KeyLength 2048 `
    -NotBefore (Get-Date) `
    -NotAfter (Get-Date).AddYears(1) `
    -CertStoreLocation "Cert:\CurrentUser\My" `
    -KeyUsage DigitalSignature, KeyEncipherment `
    -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.1")

# Экспорт сертификата
$certPassword = ConvertTo-SecureString -String "useproxy" -Force -AsPlainText
$certPath = "cert.pfx"
$cert | Export-PfxCertificate -FilePath $certPath -Password $certPassword

# Конвертация в PEM формат с помощью OpenSSL
openssl pkcs12 -in cert.pfx -out cert.pem -nodes -nokeys -password pass:useproxy
openssl pkcs12 -in cert.pfx -out key.pem -nodes -nocerts -password pass:useproxy

Write-Host "Сертификаты успешно сгенерированы:"
Write-Host "cert.pem - сертификат"
Write-Host "key.pem - приватный ключ"

# Очистка временных файлов
Remove-Item cert.pfx 