<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CREATE ProviderLocationContactType</name>
   <tag></tag>
   <elementGuidId>b13041fb-05cb-4fdf-b3e0-f67ab2fa6266</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot; {\n        \&quot;createdUserId\&quot;: 1,\n        \&quot;eTag\&quot;: 0,\n        \&quot;healthSystemId\&quot;: 106,\n        \&quot;ExternalCode\&quot;: \&quot;${UUID.randomUUID().toString()}\&quot;,\n        \&quot;dataSourceId\&quot;: 12,\n        \&quot;ProviderLocationOperatingScheduleTypeName\&quot; : \&quot;${UUID.randomUUID().toString()}\&quot;\n        }&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${AccessToken}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${HealthSystemAdmin}/ProviderLocationContactTypes</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.AccessToken</defaultValue>
      <description></description>
      <id>2e24b1bd-367f-49f8-88d2-574c90f35c65</id>
      <masked>false</masked>
      <name>AccessToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HealthSystemAdmin</defaultValue>
      <description></description>
      <id>f3cafb6c-035f-4e4d-967d-3cf613df58f0</id>
      <masked>false</masked>
      <name>HealthSystemAdmin</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
