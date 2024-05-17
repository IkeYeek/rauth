import { defineStore } from "pinia";
import { ApiService } from "@/helpers/api_service";

export type NewDomainRule = {
  domain: string;
  group_id: number;
};
export type DomainRule = NewDomainRule & {
  id: number;
};
export type NewUrlRule = {
  url: string;
  group_id: number;
};
export type UrlRule = NewUrlRule & {
  id: number;
};
export const useRulesStore = defineStore("rules", () => {
  const getDomainRules = async (): Promise<Array<DomainRule>> => {
    return await ApiService.makeAuthenticatedApiRequest<Array<DomainRule>>(
      "get",
      "api/rules/domain",
    );
  };

  const addDomainRule = async (payload: NewDomainRule): Promise<DomainRule> => {
    return await ApiService.makeAuthenticatedApiRequest<DomainRule>(
      "post",
      "api/rules/domain",
      payload,
    );
  };

  const removeDomainRule = async (rule_id: number): Promise<void> => {
    return await ApiService.makeAuthenticatedApiRequest<void>(
      "delete",
      `api/rules/domain/${rule_id}`,
    );
  };

  const domainRulesForDomain = async (domain: string): Promise<Array<DomainRule>> => {
    return await ApiService.makeAuthenticatedApiRequest<Array<DomainRule>>(
      "get",
      `api/rules/domain/for/domain/${domain}`,
    );
  };

  const domainRulesForGroup = async (group_id: number): Promise<Array<DomainRule>> => {
    return await ApiService.makeAuthenticatedApiRequest<Array<DomainRule>>(
      "get",
      `api/rules/domain/for/group/${group_id}`,
    );
  };

  const domainRulesForUser = async (user_id: number): Promise<Array<DomainRule>> => {
    return await ApiService.makeAuthenticatedApiRequest<Array<DomainRule>>(
      "get",
      `api/rules/domain/for/user/${user_id}`,
    );
  };

  const getUrlRules = async (): Promise<Array<UrlRule>> => {
    return await ApiService.makeAuthenticatedApiRequest<Array<UrlRule>>("get", "api/rules/url");
  };

  const addUrlRule = async (payload: NewUrlRule): Promise<UrlRule> => {
    return await ApiService.makeAuthenticatedApiRequest<UrlRule>("post", "api/rules/url", payload);
  };

  const removeUrlRule = async (rule_id: number): Promise<void> => {
    return await ApiService.makeAuthenticatedApiRequest<void>("delete", `api/rules/url/${rule_id}`);
  };

  const urlRulesForUrl = async (url: string): Promise<Array<UrlRule>> => {
    return await ApiService.makeAuthenticatedApiRequest<Array<UrlRule>>(
      "get",
      `api/rules/url/for/url/${url}`,
    );
  };

  const urlRulesForGroup = async (group_id: number): Promise<Array<UrlRule>> => {
    return await ApiService.makeAuthenticatedApiRequest(
      "get",
      `api/rules/url/for/group/${group_id}`,
    );
  };

  const urlRulesForUser = async (user_id: number): Promise<Array<UrlRule>> => {
    return await ApiService.makeAuthenticatedApiRequest("get", `api/rules/url/for/user/${user_id}`);
  };

  return {
    getDomainRules,
    addDomainRule,
    domainRulesForDomain,
    domainRulesForGroup,
    domainRulesForUser,
    getUrlRules,
    addUrlRule,
    urlRulesForUrl,
    urlRulesForGroup,
    urlRulesForUser,
    removeDomainRule,
    removeUrlRule,
  };
});
