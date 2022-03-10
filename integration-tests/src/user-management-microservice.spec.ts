import faker from "@faker-js/faker";
import { access } from "fs";

import { UsermanagementMicroservice, UserType, UserRole } from "../index";

describe("Basic individual client functionalities", () => {
  let email: string;
  let password: string;
  let accessToken: string;

  beforeAll(async () => {
    email = faker.internet.email();
    password = faker.internet.password();
    const refreshToken = await UsermanagementMicroservice.createAccount(
      email,
      password,
      UserType.Individual
    );
    accessToken = await UsermanagementMicroservice.getAccessToken(refreshToken);
  });

  test("Upload onboarding details", async () => {
    await UsermanagementMicroservice.updateIndividualDetails(
      accessToken,
      UserRole.Client
    );
  });
});
