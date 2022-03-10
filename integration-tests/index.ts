import faker from "@faker-js/faker";
import fetch, { Response } from "node-fetch";
import FormData from "form-data";
import { assert } from "console";

export enum DaprId {
  UserManagementMicroservice = "user-management-microservice",
  EorAdminMicroservice = "eor-admin-microservice",
}

export enum UserType {
  Individual = "individual",
  Entity = "entity",
}

export enum UserRole {
  Client = "client",
  Contractor = "contractor",
}

export enum ContentType {
  FormUrlEncoded = "application/x-www-form-urlencoded",
  MultipartFormData = "multipart/form-data",
}

const convertResponseToText = (response: Response) => {
  if (response.status == 200) {
    return response.text();
  } else {
    throw response;
  }
};

const checkResponseStatus = (response: Response) => {
  if (response.status == 200) {
    return response;
  } else {
    throw response;
  }
};

export namespace UsermanagementMicroservice {
  export const USER_MANAGEMENT_MICROSERVICE_URL = () =>
    process.env.USER_MANAGEMENT_MICROSERVICE_URL ?? "http://localhost:3500";

  export const createAccount = async (
    email: string,
    password: string,
    userRole: UserType
  ) =>
    fetch(`${USER_MANAGEMENT_MICROSERVICE_URL()}/auth/signup/${userRole}`, {
      method: "POST",
      headers: {
        "dapr-app-id": DaprId.UserManagementMicroservice,
        "Content-Type": ContentType.FormUrlEncoded,
      },
      body: new URLSearchParams({
        email,
        password,
        "confirm-password": password,
      }),
    }).then(convertResponseToText);

  export const accountLogin = async (
    email: string,
    password: string,
    userRole: UserType
  ) =>
    fetch(`${USER_MANAGEMENT_MICROSERVICE_URL()}/auth/login/${userRole}`, {
      method: "POST",
      headers: {
        "dapr-app-id": DaprId.UserManagementMicroservice,
        "Content-Type": ContentType.FormUrlEncoded,
      },
      body: new URLSearchParams({
        email,
        password,
      }),
    }).then(checkResponseStatus);

  export const getAccessToken = async (refreshToken: string) =>
    fetch(`${USER_MANAGEMENT_MICROSERVICE_URL()}/auth/access-token`, {
      method: "POST",
      headers: {
        "dapr-app-id": DaprId.UserManagementMicroservice,
        Authorization: `Bearer ${refreshToken}`,
      },
    }).then(convertResponseToText);

  export const updateIndividualDetails = async (
    accessToken: string,
    userRole: UserRole
  ) => {
    const form = new FormData();

    form.append("first-name", faker.name.firstName());
    form.append("last-name", faker.name.lastName());
    form.append("dob", faker.date.past().toISOString());
    form.append("dial-code", "+1");
    form.append("phone-number", faker.phone.phoneNumber("##########"));
    form.append("country", faker.address.country());
    form.append("city", faker.address.city());
    form.append("address", faker.address.streetAddress());
    form.append("postal-code", faker.address.zipCode());
    // Can this be arbitrary string?
    form.append("tax-id", "2304803309");
    form.append("time-zone", "US/Eastern");

    return fetch(
      `${USER_MANAGEMENT_MICROSERVICE_URL()}/onboard/individual-details/${userRole}`,
      {
        method: "POST",
        headers: {
          "dapr-app-id": DaprId.UserManagementMicroservice,
          "Content-Type": `${
            ContentType.MultipartFormData
          };  boundary=${form.getBoundary()}`,
          Authorization: `Bearer ${accessToken}`,
        },
        body: form,
      }
    ).then(checkResponseStatus);
  };

  export const addNewIndividualContractor = async (
    accessToken: string,
    email: string
  ) => {
    fetch(
      `${UsermanagementMicroservice.USER_MANAGEMENT_MICROSERVICE_URL()}/eor-admin/users/add_individual_contractor`,
      {
        method: "POST",
        headers: {
          "Content-Type": ContentType.FormUrlEncoded,
          "dapr-app-id": DaprId.UserManagementMicroservice,
          Authorization: `Bearer ${accessToken}`,
        },
        body: new URLSearchParams({
          email,
        }),
      }
    ).then(checkResponseStatus);
  };
}

export namespace AdminManagementMicroservice {
  export const ADMIN_MANAGEMENT_MICROSERVICE_URL = () =>
    process.env.ADMIN_MANAGEMENT_MICROSERVICE_URL ?? "http://localhost:3502";

  export const createAccount = async (email: string, password: string) =>
    fetch(`${ADMIN_MANAGEMENT_MICROSERVICE_URL()}/auth/signup/`, {
      method: "POST",
      headers: {
        "dapr-app-id": DaprId.EorAdminMicroservice,
        "Content-Type": ContentType.FormUrlEncoded,
      },
      body: new URLSearchParams({
        email,
        password,
        "confirm-password": password,
      }),
    }).then(convertResponseToText);

  export const accountLogin = async (
    email: string,
    password: string,
    userRole: UserType
  ) =>
    fetch(`${ADMIN_MANAGEMENT_MICROSERVICE_URL()}/auth/login/${userRole}`, {
      method: "POST",
      headers: {
        "dapr-app-id": DaprId.EorAdminMicroservice,
        "Content-Type": ContentType.FormUrlEncoded,
      },
      body: new URLSearchParams({
        email,
        password,
      }),
    }).then(checkResponseStatus);

  export const getAccessToken = async (refreshToken: string) =>
    fetch(`${ADMIN_MANAGEMENT_MICROSERVICE_URL()}/auth/access-token`, {
      method: "POST",
      headers: {
        "dapr-app-id": DaprId.EorAdminMicroservice,
        Authorization: `Bearer ${refreshToken}`,
      },
    }).then(convertResponseToText);

  export const updateAdminDetails = async (accessToken: string) => {
    const form = new FormData();

    form.append("first-name", faker.name.firstName());
    form.append("last-name", faker.name.lastName());
    form.append("dob", faker.date.past().toISOString());
    form.append("dial-code", "+1");
    form.append("phone-number", faker.phone.phoneNumber("##########"));
    form.append("country", faker.address.country());
    form.append("city", faker.address.city());
    form.append("address", faker.address.streetAddress());
    form.append("postal-code", faker.address.zipCode());
    // Can this be arbitrary string?
    form.append("tax-id", "2304803309");
    form.append("time-zone", "US/Eastern");

    return fetch(
      `${ADMIN_MANAGEMENT_MICROSERVICE_URL()}/onboard/admin-details`,
      {
        method: "POST",
        headers: {
          "dapr-app-id": DaprId.EorAdminMicroservice,
          "Content-Type": `${
            ContentType.MultipartFormData
          };  boundary=${form.getBoundary()}`,
          Authorization: `Bearer ${accessToken}`,
        },
        body: form,
      }
    ).then(checkResponseStatus);
  };
}
