<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Token</name>
   <tag></tag>
   <elementGuidId>8b524e11-a480-4744-8054-e5d4cf8011b9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;grant_type&quot;,
      &quot;value&quot;: &quot;password&quot;
    },
    {
      &quot;name&quot;: &quot;username&quot;,
      &quot;value&quot;: &quot;NileshAdmin&quot;
    },
    {
      &quot;name&quot;: &quot;password&quot;,
      &quot;value&quot;: &quot;Nilesh123#&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${Auth}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Auth</defaultValue>
      <description></description>
      <id>7852c8ee-a0d3-408c-8b6c-7bbd5f4a1efd</id>
      <masked>false</masked>
      <name>Auth</name>
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

WS.verifyElementPropertyValue(response, 'access_token', &quot;6fBabj7M4gFUiTMZZ23HaC-lSBomWFr8oPFtZ5CVz3G51Q9F86TAA4tmIQFYnTFGoQ1F_r8L7HC8iNEK5UPGKGPlEeRSnyDkcPfGwl6nV8gzWzO642SL6IpfEisN6N3qx-Vq2P1LkpRjCPJFdMyP6DVZ4Q_WxILwpND119yu-nf-CGDHnDfNFxn39xG6SsDwpMdk85gjkAHzfj5BcivjNQTyn1SaLZKH-TqTlt4nXASLCvxg8KcFvDPPfVYQ4eJkCnuQoNCLA8VrrpldJ-QvGp8CwR47zdL32XN8Ajen_JMlyqjQVTADjMNJNf3PxT0qk8N4mSM8W2WM7Y0WtpEixGIgoKYb0ZFxP7OIajMmWQ4ave7F&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
