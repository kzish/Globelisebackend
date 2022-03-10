import faker from "@faker-js/faker";

import {
  UsermanagementMicroservice,
  AdminManagementMicroservice,
} from "../index";

describe("Basic admin functionalities", () => {
  let email: string;
  let password: string;
  let refreshToken: string;
  let accessToken: string;

  beforeAll(async () => {
    email = faker.internet.email();
    password = faker.internet.password();
    refreshToken = await AdminManagementMicroservice.createAccount(
      email,
      password
    );
    accessToken = await AdminManagementMicroservice.getAccessToken(
      refreshToken
    );
  });

  test("Upload onboarding details", async () => {
    await AdminManagementMicroservice.updateAdminDetails(accessToken);
  });

  test("Add new individual contractor", async () => {
    const userEmail = faker.internet.email();
    await UsermanagementMicroservice.addNewIndividualContractor(
      accessToken,
      userEmail
    );
  });
});
