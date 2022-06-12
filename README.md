# monitor-ec2
EC2の起動状態を監視しRunning状態のインスタンスについてSlackに通知する

## incomming webhook を使うバージョン
```
$ SLACK_CHANNEL_WEBHOOK_URL=https://hooks.slack.com/services/xxxxx cargo r 
```

## api token を使うバージョン
```
$ SLACK_BOT_TOKEN="xoxb-xxxxxx" cargo r 
```
