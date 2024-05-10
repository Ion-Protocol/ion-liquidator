const marketConfig = require("./markets.json");

/**
 * @param {string} collateral
 */
const createLiquidator = (collateral) => {
  const ionLiquidatorResources = {
    [`${collateral}LiquidatorLogGroup`]: {
      Type: "AWS::Logs::LogGroup",
      Properties: {
        LogGroupName: `/ecs/${collateral.toLowerCase()}-liquidator`,
        RetentionInDays: 60,
      },
    },
    [`${collateral}LiquidatorTaskDefinition`]: {
      Type: "AWS::ECS::TaskDefinition",
      Properties: {
        Family: `${collateral.toLowerCase()}-liquidator`,
        Cpu: "1024",
        Memory: "2048",
        NetworkMode: "awsvpc",
        RequiresCompatibilities: ["FARGATE"],
        ExecutionRoleArn: "arn:aws:iam::664491262134:role/ecsTaskExecutionRole",
        TaskRoleArn: "arn:aws:iam::664491262134:role/ecsTaskRole",
        ContainerDefinitions: [
          {
            Name: `${collateral.toLowerCase()}-liquidator`,
            Image: `664491262134.dkr.ecr.us-east-1.amazonaws.com/ion-liquidator:latest`,
            Essential: true,
            LogConfiguration: {
              LogDriver: "awslogs",
              Options: {
                "awslogs-group": `/ecs/${collateral.toLowerCase()}-liquidator`,
                "awslogs-region": "us-east-1",
                "awslogs-stream-prefix": "ecs",
              },
            },
            Environment: [
              { Name: "CHAIN_ID", Value: "1" },
              {
                Name: "POOL",
                Value: marketConfig[collateral]["IonPool"],
              },
              {
                Name: "LIQUIDATION",
                Value: marketConfig[collateral]["Liquidation"],
              },
              {
                Name: "TREASURY",
                Value: marketConfig[collateral]["Treasury"],
              },
              {
                Name: "COLLATERAL",
                Value: marketConfig[collateral]["Collateral"],
              },
              {
                Name: "JOIN",
                Value: marketConfig[collateral]["GemJoin"],
              },
              {
                Name: "CLIQUIDATOR",
                Value: marketConfig[collateral]["CurveLiquidator"],
              },
              {
                Name: "ULIQUIDATOR",
                Value: marketConfig[collateral]["UniswapLiquidator"],
              },
              {
                Name: "CPOOL",
                Value: marketConfig[collateral]["CurvePool"],
              },
              {
                Name: "UPOOL",
                Value: marketConfig[collateral]["UniswapPool"],
              },
            ],
            Secrets: [
              {
                Name: "HTTP_RPC_URL",
                ValueFrom:
                  "arn:aws:secretsmanager:us-east-1:664491262134:secret:bot/config-TUnMCY:HTTP_RPC_URL::",
              },
              {
                Name: "WS_RPC_URL",
                ValueFrom:
                  "arn:aws:secretsmanager:us-east-1:664491262134:secret:bot/config-TUnMCY:WS_RPC_URL::",
              },
              {
                Name: "PRIVATE_KEY",
                ValueFrom:
                  "arn:aws:secretsmanager:us-east-1:664491262134:secret:bot/config-TUnMCY:PRIVATE_KEY::",
              },
            ],
          },
        ],
      },
    },
    [`${collateral}LiquidatorService`]: {
      Type: "AWS::ECS::Service",
      DependsOn: `${collateral}LiquidatorTaskDefinition`,
      Properties: {
        ServiceName: `${collateral.toLowerCase()}-liquidator`,
        Cluster:
          "arn:aws:ecs:us-east-1:664491262134:cluster/LiquidationsCluster",
        LaunchType: "FARGATE",
        TaskDefinition: {
          Ref: `${collateral}LiquidatorTaskDefinition`,
        },
        DesiredCount: 1,
        NetworkConfiguration: {
          AwsvpcConfiguration: {
            Subnets: ["subnet-0207c69c61a78ce59"],
            SecurityGroups: ["sg-0edc503976c71f4ea"],
            AssignPublicIp: "ENABLED",
          },
        },
      },
    },
  };

  return ionLiquidatorResources;
};

const liquidators = Object.keys(marketConfig).reduce(
  (acc, key) => ({ ...acc, ...createLiquidator(key) }),
  {}
);

module.exports = {
  Resources: liquidators,
};
